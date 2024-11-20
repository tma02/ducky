use std::{collections::HashMap, sync::LazyLock};

use steamworks::SteamId;

use crate::{
    game::{actor::{Actor, ActorType}, Game},
    packet::{
        util::validate_dict_field_types,
        variant::{Dictionary, VariantType, VariantValue},
    },
    Server,
};

static PARAMS_SCHEMA: LazyLock<HashMap<String, VariantType>> = LazyLock::new(|| {
    HashMap::from([
        ("actor_id".to_string(), VariantType::Int),
        ("actor_type".to_string(), VariantType::String),
        ("creator_id".to_string(), VariantType::Int),
        ("zone".to_string(), VariantType::String),
        ("zone_owner".to_string(), VariantType::Int),
        ("at".to_string(), VariantType::Vector3),
        ("rot".to_string(), VariantType::Vector3),
    ])
});
static TAG: &str = "instance_actor";

pub fn handle(_server: &mut Server, game: &mut Game, steam_id: SteamId, packet: Dictionary) {
    let Some(VariantValue::Dictionary(params)) = packet.get("params") else {
        println!("[{}] Missing params in instance_actor packet.", TAG);
        return;
    };

    if !validate_dict_field_types(params, &PARAMS_SCHEMA) {
        println!("[{}] Invalid params in instance_actor packet: packet = {packet:?}", TAG);
        return;
    }
    // Unwrap should be safe since we validated the fields above.
    let type_string: String = params
        .get("actor_type")
        .unwrap()
        .clone()
        .try_into()
        .unwrap();
    let actor_type = ActorType::from(type_string.as_str());
    if !game
        .actor_manager
        .user_can_create_actor(&steam_id, false, &actor_type)
    {
        println!(
            "[{}] Blocked user creating an actor: steam_id = {}, packet = {:?}",
            TAG,
            steam_id.raw(),
            packet
        );
        return;
    }
    let actor = Actor {
        id: params.get("actor_id").unwrap().clone().try_into().unwrap(),
        creator_id: steam_id,
        actor_type,
        zone: params.get("zone").unwrap().clone().try_into().unwrap(),
        zone_owner: params
            .get("zone_owner")
            .unwrap()
            .clone()
            .try_into()
            .unwrap(),
        position: params.get("at").unwrap().clone().try_into().unwrap(),
        rotation: params.get("rot").unwrap().clone().try_into().unwrap(),
    };
    println!(
        "[{}] Inserting new actor: steam_id = {} actor = {:?}",
        TAG,
        steam_id.raw(),
        actor
    );
    game.actor_manager.insert_actor(actor);
}
