use steamworks::SteamId;

use crate::{game::Game, packet::variant::Dictionary, Server};

static TAG: &str = "actor_action";

pub fn handle(_server: &mut Server, _game: &mut Game, _steam_id: SteamId, packet: Dictionary) {
    // TODO: handle this
    println!("[{TAG}] {packet:?}");
}
