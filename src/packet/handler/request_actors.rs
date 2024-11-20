use steamworks::{SendType, SteamId};

use crate::{
    game::Game, packet::{
        encode::encode_variant,
        variant::{Dictionary, VariantValue},
        OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
    }, Server
};

pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    let mut response = Dictionary::new();
    response.insert(
        "type".to_owned(),
        VariantValue::String("actor_request_send".to_string()),
    );
    response.insert("list".to_owned(), VariantValue::Array(Vec::new()));

    server
        .sender_p2p_packet
        .send(OutgoingP2pPacketRequest {
            data: encode_variant(VariantValue::Dictionary(response)),
            target: P2pPacketTarget::SteamId(steam_id),
            channel: P2pChannel::GameState,
            send_type: SendType::Reliable,
        })
        .unwrap();
}
