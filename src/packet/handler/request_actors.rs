use steamworks::{SendType, SteamId};

use crate::{
    game::Game,
    packet::{
        util::send_variant_p2p,
        variant::{Dictionary, VariantValue},
        P2pChannel, P2pPacketTarget,
    },
    Server,
};

pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    let mut response = Dictionary::new();
    response.insert(
        "type".to_owned(),
        VariantValue::String("actor_request_send".to_string()),
    );
    response.insert("list".to_owned(), VariantValue::Array(Vec::new()));

    send_variant_p2p(
        &server.sender_p2p_packet,
        VariantValue::Dictionary(response),
        P2pPacketTarget::SteamId(steam_id),
        P2pChannel::GameState,
        SendType::Reliable,
    );
}
