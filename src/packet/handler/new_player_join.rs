use steamworks::SteamId;

use crate::{game::Game, packet::variant::Dictionary, Server};

/// Responds to a new_player_join packet.
pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    server.send_chat_message(&steam_id, &server.motd);
}
