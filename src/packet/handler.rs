use steamworks::SteamId;

use crate::{game::Game, Server};

use super::variant::{Dictionary, VariantValue};

pub mod actor_action;
pub mod actor_animation_update;
pub mod actor_request_send;
pub mod actor_update;
pub mod handshake;
pub mod instance_actor;
pub mod message;
pub mod new_player_join;
pub mod request_actors;
pub mod request_ping;

/// Packet handlers are pure functions responsible for handling a single packet type. All packet
/// handlers have the same function signature `fn(&mut Server, &mut Game, SteamId, Dictionary)`.
/// This function returns the handler function for the given message root. This will return an empty
/// Option if the message did not have a type field, or if no handler could be found for the message 
/// type.
pub fn resolve_handler(
    root: &Dictionary,
) -> Option<fn(&mut Server, &mut Game, SteamId, Dictionary)> {
    let type_var = root.get("type").unwrap_or(&VariantValue::Nil);
    // TODO: Packet type registry? Not sure if needed since scope of packet types is known.
    match type_var {
        VariantValue::String(str) if str == "actor_action" => Some(actor_action::handle),
        VariantValue::String(str) if str == "actor_animation_update" => {
            Some(actor_animation_update::handle)
        }
        VariantValue::String(str) if str == "actor_request_send" => {
            Some(actor_request_send::handle)
        }
        VariantValue::String(str) if str == "actor_update" => Some(actor_update::handle),
        VariantValue::String(str) if str == "handshake" => Some(handshake::handle),
        VariantValue::String(str) if str == "instance_actor" => Some(instance_actor::handle),
        VariantValue::String(str) if str == "message" => Some(message::handle),
        VariantValue::String(str) if str == "new_player_join" => Some(new_player_join::handle),
        VariantValue::String(str) if str == "request_actors" => Some(request_actors::handle),
        VariantValue::String(str) if str == "request_ping" => Some(request_ping::handle),
        // TODO: actor_action._set_zone sets the actor zone
        _ => None,
    }
}
