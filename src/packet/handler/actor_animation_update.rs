use steamworks::SteamId;

use crate::{game::Game, packet::variant::Dictionary, Server};

pub fn handle(_server: &mut Server, _game: &mut Game, _steam_id: SteamId, _packet: Dictionary) {
    // We probably don't need to handle these, this is just to prevent logs from spamming the
    // console.
}
