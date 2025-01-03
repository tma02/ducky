use steamworks::{SendType, SteamId};

use crate::{
    game::Game,
    packet::{
        util::send_variant_p2p,
        variant::{Dictionary, VariantValue},
        P2pChannel, P2pPacketTarget,
    },
    time::system_time_since_unix_epoch_seconds_f64,
    Server,
};

pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    let mut response = Dictionary::new();
    response.insert(
        "type".to_owned(),
        VariantValue::String("send_ping".to_string()),
    );
    response.insert(
        "time".to_owned(),
        VariantValue::String(system_time_since_unix_epoch_seconds_f64().to_string()),
    );
    response.insert(
        "from".to_owned(),
        VariantValue::String(server.steam_client.user().steam_id().raw().to_string()),
    );

    send_variant_p2p(
        &server.sender_p2p_packet,
        VariantValue::Dictionary(response),
        P2pPacketTarget::SteamId(steam_id),
        P2pChannel::GameState,
        SendType::Unreliable,
    );
}
