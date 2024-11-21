use steamworks::SteamId;

use crate::{
    game::{
        actor::{Actor, ActorType},
        Game,
    },
    packet::variant::{Dictionary, VariantValue},
    Server,
};

static TAG: &str = "instance_actor";

pub fn handle(_server: &mut Server, game: &mut Game, steam_id: SteamId, mut packet: Dictionary) {
    let Some(VariantValue::Dictionary(mut params)) = packet.remove("params") else {
        println!("[{TAG}] Missing params in instance_actor packet.");
        return;
    };
    /*
    Params format:
    {
        actor_id: Int,
        actor_type: String,
        creator_id: Int,
        zone: String,
        zone_owner: Int,
        at: Vector3,
        rot: Vector3
    }
     */
    let (
        Some(VariantValue::Int(actor_id)),
        Some(VariantValue::String(actor_type)),
        Some(VariantValue::String(zone)),
        Some(VariantValue::Int(zone_owner)),
        Some(VariantValue::Vector3(position)),
        Some(VariantValue::Vector3(rotation)),
    ) = (
        params.remove("actor_id"),
        params.remove("actor_type"),
        params.remove("zone"),
        params.remove("zone_owner"),
        params.remove("at"),
        params.remove("rot"),
    )
    else {
        println!(
            "[{TAG}] Invalid params in instance_actor packet: steam_id = {} params = {params:?}",
            steam_id.raw()
        );
        return;
    };
    let actor_type = ActorType::from(actor_type.as_str());
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
        id: actor_id,
        creator_id: steam_id,
        actor_type,
        zone: zone,
        zone_owner: zone_owner,
        position: position,
        rotation: rotation,
    };
    println!(
        "[{}] Inserting new actor: steam_id = {} actor = {:?}",
        TAG,
        steam_id.raw(),
        actor
    );
    game.actor_manager.insert_actor(actor);
}
