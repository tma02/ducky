use steamworks::SteamId;

use crate::{
    game::Game,
    packet::variant::{Array, Dictionary, VariantValue},
    Server,
};

static TAG: &str = "actor_action";

pub fn handle(server: &mut Server, game: &mut Game, steam_id: SteamId, mut packet: Dictionary) {
    let (
        Some(VariantValue::String(action)),
        Some(VariantValue::Int(actor_id)),
        Some(VariantValue::Array(params)),
    ) = (
        packet.remove("action"),
        packet.remove("actor_id"),
        packet.remove("params"),
    )
    else {
        println!("[{TAG}] Ignoring invalid actor_action packet: packet = {packet:?}");
        return;
    };

    let Some(action_fn) = resolve_action_handler(action.as_str()) else {
        println!(
            "[{TAG}] Ignoring actor_action packet without action handler: packet = {packet:?}"
        );
        return;
    };

    action_fn(server, game, steam_id, actor_id, params);
}

fn resolve_action_handler(
    action: &str,
) -> Option<fn(&mut Server, &mut Game, SteamId, i64, Array)> {
    // A limitation of this approach is that we can't handle actions that have the same name but
    // belong to different actor classes.
    match action {
        "_wipe_actor" => Some(wipe_actor),
        "_set_zone" => Some(set_zone),
        // _change_id is only sent for AquaFish to update the type of fish for the actor. We don't
        // hold any state for this, so we can safely ignore it.
        "_change_id" => Some(no_op_action),
        "_play_particle" | "_play_sfx" | "_update_held_item" | "_update_cosmetics" => {
            Some(no_op_action)
        }
        _ => None,
    }
}

fn no_op_action(
    _server: &mut Server,
    _game: &mut Game,
    _steam_id: SteamId,
    _actor_id: i64,
    _params: Array,
) {
}

fn wipe_actor(
    server: &mut Server,
    game: &mut Game,
    _steam_id: SteamId,
    _actor_id: i64,
    params: Array,
) {
    let Some(VariantValue::Int(target_id)) = params.get(0) else {
        println!("[{TAG}] Ignoring invalid _wipe_actor packet: params = {params:?}");
        return;
    };

    // `actor_id` is the of the actor who initiated the wipe, not the target actor to wipe. All
    // clients will receive this action, but only the actor owner will handle despawning the actor.
    // If we are the owner of `target_id`, despawn the actor.

    let actor_manager = &mut game.actor_manager;

    let Some(host_actor) = actor_manager.get_actor(target_id).and_then(|actor| {
        if actor.creator_id == server.steam_client.user().steam_id() {
            Some(actor.id)
        } else {
            None
        }
    }) else {
        // We don't own the target actor, we can ignore.
        return;
    };

    // TODO: Check if the player is able to wipe this actor.
    actor_manager.despawn_host_actor(&server.sender_p2p_packet, &host_actor);

    println!("[{TAG}] wipe_actor: id = {target_id}");
}

fn set_zone(
    _server: &mut Server,
    game: &mut Game,
    steam_id: SteamId,
    actor_id: i64,
    mut params: Array,
) {
    let (Some(VariantValue::String(zone)), Some(VariantValue::Int(zone_owner))) =
        (params.pop(), params.pop())
    else {
        println!("[{TAG}] Ignoring invalid _set_zone packet: params = {params:?}");
        return;
    };

    let actor_manager = &mut game.actor_manager;

    println!("[{TAG}] set_zone: id = {actor_id}, zone = {zone}, zone_owner = {zone_owner}");

    let owned_by_steam_id = actor_manager
        .get_actor(&actor_id)
        .map(|a| a.creator_id == steam_id)
        .unwrap_or(false);
    if !owned_by_steam_id {
        // User doesn't own the target actor, we can ignore.
        println!(
            "[{TAG}] Ignoring _set_zone packet for actor not owned by sender: params = {params:?}"
        );
        return;
    }
    let Some(_) = actor_manager.set_actor_zone(&actor_id, zone, zone_owner) else {
        println!("[{TAG}] Failed _set_zone packet: params = {params:?}");
        return;
    };
}
