use steamworks::SteamId;

use crate::{
    game::{
        actor::{Actor, ActorType},
        Game,
    },
    packet::variant::{Dictionary, VariantValue, Vector3},
    Server,
};

static TAG: &str = "actor_request_send";

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
    /*
    Dictionary format:
    {
        id: Int,
        type: String,
        owner: Int,
    }
    */
    let (Some(VariantValue::String(type_string)), Some(VariantValue::Int(id))) =
        (actor_dict.get("type"), actor_dict.get("id"))
    else {
        println!("[{TAG}] Invalid actor: dict = {actor_dict:?}");
        return;
    };
    let actor_type = ActorType::from(type_string.as_str());
    let actor = Actor {
        id: *id,
        creator_id: steam_id.clone(),
        actor_type,
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
    if !game
        .actor_manager
        .user_can_create_actor(&steam_id, false, &actor.actor_type)
    {
        println!("[{TAG}] Blocked user actor replication: actor = {actor:?}");
        return;
    }
    
    println!("[{TAG}] Inserting actor: actor = {actor:?}");
    game.actor_manager.insert_actor(actor);
}
