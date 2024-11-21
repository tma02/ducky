use std::{collections::HashMap, sync::LazyLock};

use steamworks::SteamId;

use crate::{
    game::{
        actor::{Actor, ActorType},
        Game,
    },
    packet::{
        util::validate_dict_field_types,
        variant::{Dictionary, VariantType, VariantValue, Vector3},
    },
    Server,
};

static TAG: &str = "actor_request_send";
static ACTOR_SCHEMA: LazyLock<HashMap<String, VariantType>> = LazyLock::new(|| {
    HashMap::from([
        ("id".to_string(), VariantType::Int),
        ("type".to_string(), VariantType::String),
        ("owner".to_string(), VariantType::Int),
    ])
});

pub fn handle(_server: &mut Server, game: &mut Game, steam_id: SteamId, packet: Dictionary) {
    let Some(VariantValue::Array(list)) = packet.get("list") else {
        println!("[{TAG}] Missing list in actor_request_send: packet = {packet:?}");
        return;
    };

    list.iter().for_each(|d| {
        if let VariantValue::Dictionary(d) = d {
            insert_actor_from_list(game, &steam_id, d);
        }
    });
}

fn insert_actor_from_list(game: &mut Game, steam_id: &SteamId, actor_dict: &Dictionary) {
    if !validate_dict_field_types(actor_dict, &ACTOR_SCHEMA) {
        println!("[{TAG}] Invalid actor: dict = {actor_dict:?}");
        return;
    }
    // Unwrap should be safe since we validated the fields above.
    let type_string: String = actor_dict.get("type").unwrap().clone().try_into().unwrap();
    let actor_type = ActorType::from(type_string.as_str());
    let actor = Actor {
        id: actor_dict.get("id").unwrap().clone().try_into().unwrap(),
        creator_id: steam_id.clone(),
        actor_type: actor_type,
        zone: "".to_owned(),
        zone_owner: -1,
        position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };
    println!("[{TAG}] Inserting actor: actor = {actor:?}");
    game.actor_manager.insert_actor(actor);
}
