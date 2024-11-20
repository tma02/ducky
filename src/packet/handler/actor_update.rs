use std::{collections::HashMap, sync::LazyLock};

use steamworks::{SendType, SteamId};

use crate::{
    game::Game,
    packet::{
        util::{build_actor_request_packet, validate_dict_field_types},
        variant::{Dictionary, VariantType, Vector3},
        OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
    },
    Server,
};

static TAG: &str = "actor_update";
static PACKET_SCHEMA: LazyLock<HashMap<String, VariantType>> = LazyLock::new(|| {
    HashMap::from([
        ("actor_id".to_string(), VariantType::Int),
        ("pos".to_string(), VariantType::Vector3),
        ("rot".to_string(), VariantType::Vector3),
    ])
});

pub fn handle(server: &mut Server, game: &mut Game, steam_id: SteamId, packet: Dictionary) {
    if !validate_dict_field_types(&packet, &PACKET_SCHEMA) {
        println!(
            "[{TAG}] Ignoring invalid actor_update packet: steam_id = {} packet = {:?}",
            steam_id.raw(),
            packet
        );
        return;
    }
    let actor_id: i64 = packet.get("actor_id").unwrap().clone().try_into().unwrap();
    if let Some(actor) = game.actor_manager.get_actor_mut(&actor_id) {
        if actor.creator_id != steam_id {
            println!(
                "[{TAG}] Ignoring actor_update packet from {} for actor {} they do not own",
                steam_id.raw(),
                actor_id
            );
            return;
        }
        let pos: &Vector3 = packet.get("pos").unwrap().try_into().unwrap();
        let rot: &Vector3 = packet.get("rot").unwrap().try_into().unwrap();
        actor.position.x = pos.x;
        actor.position.y = pos.y;
        actor.position.z = pos.z;
        actor.rotation.x = rot.x;
        actor.rotation.y = rot.y;
        actor.rotation.z = rot.z;
    } else {
        let _ = server.sender_p2p_packet.send(OutgoingP2pPacketRequest {
            data: build_actor_request_packet(server.steam_client.user().steam_id()),
            target: P2pPacketTarget::SteamId(steam_id),
            channel: P2pChannel::GameState,
            send_type: SendType::Reliable,
        });
        println!(
            "[{TAG}] Ignoring actor_update packet from {} for non-existent actor {}",
            steam_id.raw(),
            actor_id
        );
    }
}
