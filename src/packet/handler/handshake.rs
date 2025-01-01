use steamworks::{SendType, SteamId};

use crate::{
    game::Game,
    packet::{
        util::{build_weblobby_packet, send_variant_p2p},
        variant::Dictionary,
        P2pChannel, P2pPacketTarget,
    },
    Server,
};

static TAG: &str = "handshake";

pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    println!(
        "[{}] Received handshake from: steam_id = {}",
        TAG,
        steam_id.raw()
    );
    send_variant_p2p(
        &server.sender_p2p_packet,
        build_weblobby_packet(&server.users),
        P2pPacketTarget::All,
        P2pChannel::GameState,
        SendType::Reliable,
    );
}
