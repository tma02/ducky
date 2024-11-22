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

pub fn handle(server: &mut Server, game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    let mut response = Dictionary::new();
    response.insert(
        "type".to_owned(),
        VariantValue::String("actor_request_send".to_string()),
    );
    let actors = game
        .actor_manager
        .get_actors_by_creator(&server.steam_client.user().steam_id());
    let mut list = Vec::new();
    for actor in actors {
        list.push(actor.clone_to_replication_variant_dict());
    }
    response.insert("list".to_owned(), VariantValue::Array(list));

    send_variant_p2p(
        &server.sender_p2p_packet,
        VariantValue::Dictionary(response),
        P2pPacketTarget::SteamId(steam_id),
        P2pChannel::GameState,
        SendType::Reliable,
    );
}
