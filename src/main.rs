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
    util::{
        build_force_disconnect_player_packet, build_handshake_packet,
        build_user_joined_weblobby_packet, send_variant_p2p,
    },
    OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
};
use server::Server;
use steamworks::{
    networking_messages::SessionRequest, ChatMemberStateChange, Client, ClientManager,
    LobbyChatMsg, LobbyChatUpdate, LobbyId, LobbyType, Matchmaking, SendType,
};
use time::system_time_since_unix_epoch_seconds;

mod command;
mod config;
mod game;
mod packet;
mod random;
mod server;
mod time;

static TAG: &str = "ducky";
static WF_APP_ID: u32 = 3146520;
static TICK_MS: u128 = 1000 / 16; // 16 ticks/s
static LOBBY_UPDATE_INTERVAL_SEC: u64 = 20; // 20 seconds

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
    let (sender_create_lobby, receiver_create_lobby) = mpsc::channel();
    let (sender_lobby_chat_update, receiver_lobby_chat_update) = mpsc::channel();
    let (sender_lobby_chat_msg, receiver_lobby_chat_msg) = mpsc::channel();
    let (sender_net_session, receiver_net_session) = mpsc::channel();
    init_steam_networking(&client, sender_net_session);
    init_lobby(
        &client,
        &config,
        sender_create_lobby,
        sender_lobby_chat_update,
        sender_lobby_chat_msg,
    );

    let (sender_p2p_packet, receiver_p2p_packet) = mpsc::channel::<OutgoingP2pPacketRequest>();
    let matchmaking = client.matchmaking();
    let networking_messages = client.networking_messages();
    let mut server = Server::new(client, sender_p2p_packet, config.clone());
    server
        .users
        .insert(server.steam_client.user().steam_id().raw());
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
            set_lobby_data(new_lobby_id, &matchmaking, 1, &config);
        }
        while let Ok(update) = receiver_lobby_chat_update.try_recv() {
            on_lobby_chat_update(&mut server, &mut game, update);
        }
        while let Ok(msg) = receiver_lobby_chat_msg.try_recv() {
            on_lobby_chat_msg(&mut server, msg);
        }
        while let Ok(session_request) = receiver_net_session.try_recv() {
            on_net_session_request(&mut server, session_request);
        }
        while let Ok(outgoing) = receiver_p2p_packet.try_recv() {
            on_send_packet(&server, outgoing);
        }

        if lobby_update_timer.elapsed() > Duration::from_secs(LOBBY_UPDATE_INTERVAL_SEC) {
            if let Some(lobby_id) = server.lobby_id {
                lobby_update_timer = Instant::now();
                set_lobby_data(lobby_id, &matchmaking, server.users.len(), &config);
            }
        }

        server.steam_client.run_callbacks();
        for channel in P2pChannel::VALUES {
            let channel_u32 = channel as u32;
            loop {
                let received = networking_messages.receive_messages_on_channel(channel_u32, 1);
                if received.len() == 0 {
                    break;
                }
                for message in received {
                    if let Some(sender) = message.identity_peer().steam_id() {
                        on_receive_packet(&mut server, &mut game, message.data().to_vec(), sender);
                    }
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

fn init_steam_networking(
    client: &Client,
    sender_net_session: Sender<SessionRequest<ClientManager>>,
) {
    client.networking_utils().init_relay_network_access();
    client
        .networking_messages()
        .session_request_callback(move |request| {
            let _ = sender_net_session.send(request);
        });
}

fn init_lobby(
    client: &Client,
    config: &Config,
    sender_create_lobby: Sender<LobbyId>,
    sender_lobby_chat_update: Sender<LobbyChatUpdate>,
    sender_lobby_chat_msg: Sender<LobbyChatMsg>,
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

    client.register_callback(move |msg: LobbyChatMsg| {
        sender_lobby_chat_msg.send(msg).unwrap();
    });
}

fn set_lobby_data(
    lobby_id: LobbyId,
    matchmaking: &Matchmaking<ClientManager>,
    user_count: usize,
    config: &Config,
) {
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
        "tag_talkative",
        &Config::get_lobby_data_for_bool(config.tag_talkative),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_quiet",
        &Config::get_lobby_data_for_bool(config.tag_quiet),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_grinding",
        &Config::get_lobby_data_for_bool(config.tag_grinding),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_chill",
        &Config::get_lobby_data_for_bool(config.tag_chill),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_silly",
        &Config::get_lobby_data_for_bool(config.tag_silly),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_hardcore",
        &Config::get_lobby_data_for_bool(config.tag_hardcore),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_mature",
        &Config::get_lobby_data_for_bool(config.tag_mature),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "tag_modded",
        &Config::get_lobby_data_for_bool(config.tag_modded),
    );
    matchmaking.set_lobby_data(lobby_id, "request", "false");
    matchmaking.set_lobby_data(
        lobby_id,
        "timestamp",
        system_time_since_unix_epoch_seconds().to_string().as_str(),
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "type",
        if config.unlisted {
            "unlisted"
        } else {
            "public"
        },
    );
    matchmaking.set_lobby_data(
        lobby_id,
        "public",
        if config.unlisted { "false" } else { "true" },
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
    matchmaking.set_lobby_data(lobby_id, "cap", config.max_players.to_string().as_str());
    matchmaking.set_lobby_data(lobby_id, "count", user_count.to_string().as_str());
    matchmaking.set_lobby_data(lobby_id, "server_browser_value", "0");
    matchmaking.set_lobby_data(lobby_id, "lurefilter", "dedicated");

    let _ = matchmaking.send_lobby_chat_message(lobby_id, "^^duckyy_heartbeat".as_bytes());
}

fn on_lobby_chat_update(server: &mut Server, game: &mut Game, update: LobbyChatUpdate) {
    if server
        .lobby_id
        .map(|lobby_id| lobby_id != update.lobby)
        // Optional is None if we don't have a lobby yet
        .unwrap_or(true)
    {
        return;
    }
    println!(
        "[{}] Lobby update: user_changed = {}, change = {:?}, making_change = {}",
        TAG,
        update.user_changed.raw(),
        update.member_state_change,
        update.making_change.raw(),
    );
    if update.member_state_change == ChatMemberStateChange::Left
        || update.member_state_change == ChatMemberStateChange::Disconnected
        || update.member_state_change == ChatMemberStateChange::Kicked
        || update.member_state_change == ChatMemberStateChange::Banned
    {
        game.actor_manager
            .remove_all_actors_by_creator(&update.user_changed);
        server.users.remove(&update.making_change.raw());
        // We don't close any sessions here since the rust bindings doesn't expose a way to do this.
        // The session should timeout anyway after a few minutes.
    } else if update.member_state_change == ChatMemberStateChange::Entered {
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

fn on_lobby_chat_msg(server: &mut Server, msg: LobbyChatMsg) {
    let steam_id_u64 = msg.user.raw();
    let lobby_id = msg.lobby;
    println!("[{}] Lobby message: steam_id = {}", TAG, steam_id_u64);
    let mut buffer = [0u8; 1024];
    server
        .steam_client
        .matchmaking()
        .get_lobby_chat_entry(lobby_id, msg.chat_id, &mut buffer);
    let chat_text = String::from_utf8_lossy(&buffer).into_owned();
    println!(
        "[{}] Lobby message from {}: {}",
        TAG, steam_id_u64, chat_text
    );
    if chat_text.trim_matches(char::from(0)) == "$weblobby_join_request" {
        if server.ban_list.contains(&steam_id_u64) {
            let msg = format!("$weblobby_request_denied_deny-{}", steam_id_u64);
            let _ = server
                .steam_client
                .matchmaking()
                .send_lobby_chat_message(lobby_id, msg.as_bytes());
            return;
        }
        if server.users.len() as u32 >= server.config.max_players {
            let msg = format!("$weblobby_request_denied_full-{}", steam_id_u64);
            let _ = server
                .steam_client
                .matchmaking()
                .send_lobby_chat_message(lobby_id, msg.as_bytes());
            return;
        }
        server.users.insert(steam_id_u64);
        let msg = format!("$weblobby_request_accepted-{}", steam_id_u64);
        let _ = server
            .steam_client
            .matchmaking()
            .send_lobby_chat_message(lobby_id, msg.as_bytes());
        send_variant_p2p(
            &server.sender_p2p_packet,
            build_user_joined_weblobby_packet(steam_id_u64),
            P2pPacketTarget::All,
            P2pChannel::GameState,
            SendType::Reliable,
        );
    }
}

fn on_net_session_request(server: &mut Server, request: SessionRequest<ClientManager>) {
    println!("[{}] Session request received...", TAG);
    let steam_id = request.remote().steam_id();
    let Some(steam_id) = steam_id else {
        request.reject();
        return;
    };
    println!("[{}] Session request: steam_id = {}", TAG, steam_id.raw());
    // Check for reasons to not accept the request.
    if server.banned_steam_id(&steam_id) {
        println!(
            "[{}] Blocking session request from user on ban list: steam_id = {}",
            TAG,
            steam_id.raw()
        );
        request.reject();
        return;
    }
    // Checks have passed, let's accept the request
    println!(
        "[{}] Accepting session request: steam_id = {}",
        TAG,
        steam_id.raw()
    );
    request.accept();

    // Send the handshake
    send_variant_p2p(
        &server.sender_p2p_packet,
        build_handshake_packet(server.steam_client.user().steam_id()),
        P2pPacketTarget::All,
        P2pChannel::GameState,
        SendType::Reliable,
    );
}
