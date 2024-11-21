use steamworks::{SendType, SteamId};

use crate::{
    game::Game, packet::{
        util::build_message_packet, variant::Dictionary, OutgoingP2pPacketRequest, P2pChannel,
        P2pPacketTarget,
    }, Server
};

/// Responds to a new_player_join packet.
pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    server
        .sender_p2p_packet
        .send(OutgoingP2pPacketRequest {
            data: build_message_packet(&server.motd),
            target: P2pPacketTarget::SteamId(steam_id),
            channel: P2pChannel::GameState,
            send_type: SendType::Reliable,
        })
        .unwrap();
}
