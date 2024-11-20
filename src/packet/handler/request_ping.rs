use std::time::{SystemTime, UNIX_EPOCH};

use steamworks::{SendType, SteamId};

use crate::{
    game::Game, packet::{
        encode::encode_variant,
        variant::{Dictionary, VariantValue},
        OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
    }, Server
};

pub fn handle(server: &mut Server, _game: &mut Game, steam_id: SteamId, _packet: Dictionary) {
    // TODO: Move response building into private function
    let mut response = Dictionary::new();
    response.insert(
        "type".to_owned(),
        VariantValue::String("send_ping".to_string()),
    );
    response.insert(
        "time".to_owned(),
        VariantValue::String(
            // TODO: Move getting system time into a util? This is currently the only usage though.
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
                .to_string(),
        ),
    );
    response.insert(
        "from".to_owned(),
        VariantValue::String(server.steam_client.user().steam_id().raw().to_string()),
    );

    server
        .sender_p2p_packet
        .send(OutgoingP2pPacketRequest {
            data: encode_variant(VariantValue::Dictionary(response)),
            target: P2pPacketTarget::SteamId(steam_id),
            channel: P2pChannel::GameState,
            send_type: SendType::Unreliable,
        })
        .unwrap();
}
