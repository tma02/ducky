use steamworks::SteamId;

use crate::{game::Game, packet::variant::Dictionary, Server};

static TAG: &str = "handshake";

pub fn handle(_server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    println!(
        "[{}] Received handshake from: steam_id = {}",
        TAG,
        steam_id.raw()
    );
}
