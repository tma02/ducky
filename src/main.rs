use std::{
    fs, io,
    sync::mpsc::{self, Sender},
    thread,
    time::{Duration, Instant},
};

use config::Config;
use game::Game;
use packet::{
    on_receive_packet, on_send_packet,
    util::{build_force_disconnect_player_packet, build_handshake_packet, send_variant_p2p},
    OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
};
use server::Server;
use steamworks::{
    ChatMemberStateChange, Client, ClientManager, LobbyChatUpdate, LobbyId, LobbyType, Matchmaking,
    P2PSessionRequest, SendType, SteamId,
};

mod command;
mod config;
mod game;
mod packet;
mod random;
mod server;

static TAG: &str = "ducky";
static WF_APP_ID: u32 = 3146520;
static TICK_MS: u128 = 1000 / 16; // 16 ticks/s
static LOBBY_UPDATE_INTERVAL_SEC: u64 = 300; // 5 minutes

fn main() {
    let server_epoch = Instant::now();
    println!("(o< (o< (o< (o< (o<\n<_) <_) <_) <_) <_)");

    let config = match read_config() {
        Ok(config) => config,
        Err(e) => {
            println!("[{TAG}] Failed reading config.toml, using defaults. error = {e}");
            Config::default()
        }
    };
    println!("[{TAG}] Using config: config = {config:?}");

    let client = init_steam_client();
    let (sender_p2p_request, receiver_p2p_request) = mpsc::channel();
    init_steam_networking(&client, sender_p2p_request);

    let (sender_create_lobby, receiver_create_lobby) = mpsc::channel();
    let (sender_lobby_chat_update, receiver_lobby_chat_update) = mpsc::channel();
    init_lobby(
        &client,
        &config,
        sender_create_lobby,
        sender_lobby_chat_update,
    );

    let (sender_p2p_packet, receiver_p2p_packet) = mpsc::channel::<OutgoingP2pPacketRequest>();
    let matchmaking = client.matchmaking();
    let networking = client.networking();
    let mut server = Server::new(client, sender_p2p_packet, config.motd.clone());
    config
        .ban_list
        .iter()
        .for_each(|id| server.insert_ban_list(*id));

    let mut game = Game::new();
    game.on_ready(&mut server);

    let mut lobby_update_timer = Instant::now();

    loop {
        while let Ok(new_lobby_id) = receiver_create_lobby.try_recv() {
            // On lobby created
            server.set_lobby_id(new_lobby_id);
            set_lobby_data(new_lobby_id, &matchmaking, &config);
        }
        while let Ok(update) = receiver_lobby_chat_update.try_recv() {
            on_lobby_chat_update(&server, &mut game, update);
        }
        while let Ok(steam_id) = receiver_p2p_request.try_recv() {
            on_p2p_session_request(&server, steam_id.clone());
        }
        while let Ok(outgoing) = receiver_p2p_packet.try_recv() {
            on_send_packet(&server, outgoing);
        }

        if lobby_update_timer.elapsed() > Duration::from_secs(LOBBY_UPDATE_INTERVAL_SEC) {
            if let Some(lobby_id) = server.lobby_id {
                lobby_update_timer = Instant::now();
                set_lobby_data(lobby_id, &matchmaking, &config);
            }
        }

        server.steam_client.run_callbacks();
        for channel in P2pChannel::VALUES {
            let channel_i32 = channel as i32;
            while let Some(size) = networking.is_p2p_packet_available_on_channel(channel_i32) {
                let mut buffer_vec = vec![0; size];
                let buffer = buffer_vec.as_mut_slice();
                if let Some((sender, _)) =
                    networking.read_p2p_packet_from_channel(buffer, channel_i32)
                {
                    on_receive_packet(&mut server, &mut game, buffer_vec, sender);
                }
            }
        }

        game.on_update(&mut server);

        // TODO: Move loops into separate tasks or threads so they can all poll, only the Steam
        //  networking thread needs to sleep.
        let duration = TICK_MS - Instant::now().duration_since(server_epoch).as_millis() % TICK_MS;
        thread::sleep(Duration::from_millis(duration as u64));
    }
}

fn read_config() -> io::Result<Config> {
    toml::from_str(&fs::read_to_string("config.toml")?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

fn init_steam_client() -> Client {
    println!("[{}] Initializing Steam...", TAG);

    let client = Client::init_app(WF_APP_ID)
        .expect("Steam is not detected or account does not own WEBFISHING.");

    println!("[{}] Steam OK", TAG);
    client
}

fn init_steam_networking(client: &Client, sender: Sender<SteamId>) {
    client.networking_utils().init_relay_network_access();

    client.register_callback(move |request: P2PSessionRequest| {
        println!("[{}] Got P2PSessionRequest", TAG);
        sender.send(request.remote).unwrap();
    });
}

fn init_lobby(
    client: &Client,
    config: &Config,
    sender_create_lobby: Sender<LobbyId>,
    sender_lobby_chat_update: Sender<LobbyChatUpdate>,
) {
    println!("[{}] Creating Steam lobby...", TAG);

    client
        .matchmaking()
        .create_lobby(
            LobbyType::Public,
            config.max_players,
            move |result| match result {
                Ok(lobby_id) => {
                    sender_create_lobby.send(lobby_id).unwrap();
                    println!(
                        "[{}] Steam lobby created: lobby_id = {}",
                        TAG,
                        lobby_id.raw()
                    )
                }
                Err(err) => panic!("[{}] Failed to create lobby: {}", TAG, err),
            },
        );

    client.register_callback(move |update: LobbyChatUpdate| {
        sender_lobby_chat_update.send(update).unwrap();
    });
}

fn set_lobby_data(lobby_id: LobbyId, matchmaking: &Matchmaking<ClientManager>, config: &Config) {
    println!(
        "[{}] Setting lobby fields: lobby_id = {}",
        TAG,
        lobby_id.raw()
    );
    println!("[{}] Lobby code: {}", TAG, config.lobby_code);

    // Always joinable
    matchmaking.set_lobby_joinable(lobby_id, true);
    matchmaking.set_lobby_data(lobby_id, "lobby_name", &config.name);
    matchmaking.set_lobby_data(lobby_id, "ref", "webfishing_gamelobby");
    matchmaking.set_lobby_data(lobby_id, "version", &config.game_version);
    matchmaking.set_lobby_data(lobby_id, "code", &config.lobby_code);
    matchmaking.set_lobby_data(
        lobby_id,
        "type",
        if config.code_only {
            "code_only"
        } else {
            "public"
        },
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "public",
        if config.code_only { "false" } else { "true" },
    );
    // This is a CSV of SteamIDs
    matchmaking.set_lobby_data(
        lobby_id,
        "banned_players",
        &config
            .ban_list
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(","),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "age_limit",
        if config.adult_only { "true" } else { "false" },
    );
    matchmaking.set_lobby_data(lobby_id, "cap", config.max_players.to_string().as_str());
    matchmaking.set_lobby_data(
        lobby_id,
        "server_browser_value",
        &random::lobby_server_browser_value(),
    );
    matchmaking.set_lobby_data(lobby_id, "lurefilter", "dedicated");
}

fn on_lobby_chat_update(server: &Server, game: &mut Game, update: LobbyChatUpdate) {
    if server
        .lobby_id
        .map(|lobby_id| lobby_id != update.lobby)
        // Optional is None if we don't have a lobby yet
        .unwrap_or(true)
    {
        return;
    }
    if update.member_state_change == ChatMemberStateChange::Left
        || update.member_state_change == ChatMemberStateChange::Disconnected
        || update.member_state_change == ChatMemberStateChange::Kicked
        || update.member_state_change == ChatMemberStateChange::Banned
    {
        println!(
            "[{}] User left lobby: steam_id = {}",
            TAG,
            update.user_changed.raw()
        );
        server
            .steam_client
            .networking()
            .close_p2p_session(update.user_changed);
        game.actor_manager
            .remove_all_actors_by_creator(&update.user_changed);
    } else if update.member_state_change == ChatMemberStateChange::Entered {
        println!(
            "[{}] User joined lobby: steam_id = {}",
            TAG,
            update.user_changed.raw()
        );
        
        if server.banned_steam_id(&update.user_changed) {
            println!(
                "[{}] Sending force_disconnect_player packet to block P2P on players: steam_id = {}",
                TAG,
                update.user_changed.raw()
            );

            send_variant_p2p(
                &server.sender_p2p_packet,
                build_force_disconnect_player_packet(&update.user_changed.raw()),
                P2pPacketTarget::All,
                P2pChannel::GameState,
                SendType::Reliable,
            );
            return;
        }
    }
}

fn on_p2p_session_request(server: &Server, steam_id: SteamId) {
    println!("[{}] P2P request: steam_id = {}", TAG, steam_id.raw());
    // Check for reasons to not accept the request.
    if server.banned_steam_id(&steam_id) {
        println!(
            "[{}] Blocking P2P request from user on ban list: steam_id = {}",
            TAG,
            steam_id.raw()
        );
        return;
    }
    // Checks have passed, let's accept the request
    println!(
        "[{}] Accepting P2P request: steam_id = {}",
        TAG,
        steam_id.raw()
    );
    server
        .steam_client
        .networking()
        .accept_p2p_session(steam_id);

    // Send the handshake
    send_variant_p2p(
        &server.sender_p2p_packet,
        build_handshake_packet(server.steam_client.user().steam_id()),
        P2pPacketTarget::All,
        P2pChannel::GameState,
        SendType::Reliable,
    );
}
