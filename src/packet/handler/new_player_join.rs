use steamworks::SteamId;

use crate::{
    game::Game,
    packet::{variant::Dictionary, P2pPacketTarget},
    Server,
};

/// Responds to a new_player_join packet.
pub fn handle(server: &mut Server, game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    server.send_chat_message(&steam_id, &server.motd);
    game
        .actor_manager
        .sync_all_actors(server, P2pPacketTarget::SteamId(steam_id));
}
