use steamworks::SteamId;

use crate::{game::Game, packet::variant::Dictionary, Server};

static TAG: &str = "actor_animation_update";

pub fn handle(_server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    // We probably don't need to handle these, this is just to prevent logs from spamming the
    // console.
}
