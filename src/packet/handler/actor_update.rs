use steamworks::SteamId;

use crate::{
    game::Game,
    packet::variant::{Dictionary, VariantValue},
    Server,
};

static TAG: &str = "actor_update";

pub fn handle(_server: &mut Server, game: &mut Game, steam_id: SteamId, packet: Dictionary) {
    /*
    Packet format:
    {
        actor_id: Int,
        pos: Vector3,
        rot: Vector3,
    }
    */
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
