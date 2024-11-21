use std::{collections::HashMap, sync::LazyLock};

use steamworks::SteamId;

use crate::{
    game::Game,
    packet::{
        util::validate_dict_field_types,
        variant::{Dictionary, VariantType, VariantValue},
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

pub fn handle(_server: &mut Server, game: &mut Game, steam_id: SteamId, packet: Dictionary) {
    if !validate_dict_field_types(&packet, &PACKET_SCHEMA) {
        println!(
            "[{TAG}] Ignoring invalid actor_update packet: steam_id = {} packet = {:?}",
            steam_id.raw(),
            packet
        );
        return;
    }
    let (
        Some(VariantValue::Int(actor_id)),
        Some(VariantValue::Vector3(pos)),
        Some(VariantValue::Vector3(rot)),
    ) = (packet.get("actor_id"), packet.get("pos"), packet.get("rot"))
    else {
        println!(
            "[{TAG}] Ignoring invalid actor_update packet: steam_id = {} packet = {:?}",
            steam_id.raw(),
            packet
        );
        return;
    };
    if let Some(actor) = game.actor_manager.get_actor_mut(&actor_id) {
        if actor.creator_id != steam_id {
            println!(
                "[{TAG}] Ignoring actor_update packet from {} for actor {} they do not own",
                steam_id.raw(),
                actor_id
            );
            return;
        }
        actor.position.x = pos.x;
        actor.position.y = pos.y;
        actor.position.z = pos.z;
        actor.rotation.x = rot.x;
        actor.rotation.y = rot.y;
        actor.rotation.z = rot.z;
    } else {
        game.peer_manager.add_peer_need_update(steam_id);
        println!(
            "[{TAG}] Ignoring actor_update packet from {} for non-existent actor {}",
            steam_id.raw(),
            actor_id
        );
    }
}
