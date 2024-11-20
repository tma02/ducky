use std::collections::HashMap;

use steamworks::SteamId;

use crate::game::actor::Actor;

use super::{
    encode::encode_variant,
    variant::{Array, Dictionary, VariantType, VariantValue, Vector3},
};

/// Checks that the `dictionary` contains the fields and types specified in the `types_map`.
pub fn validate_dict_field_types(
    dictionary: &Dictionary,
    types_map: &HashMap<String, VariantType>,
) -> bool {
    for (key, expected_type) in types_map {
        if let Some(value) = dictionary.get(key) {
            if !value.is_type_of(*expected_type) {
                // Wrong type
                return false;
            }
        } else {
            // Missing field
            return false;
        }
    }

    true
}

/// Builds a `message` packet. This packet represents a chat message.
pub fn build_message_packet(message: &str) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert(
        "type".to_owned(),
        VariantValue::String("message".to_string()),
    );
    packet.insert(
        "message".to_owned(),
        VariantValue::String(message.to_string()),
    );
    packet.insert(
        "color".to_owned(),
        VariantValue::String("ffffff".to_string()),
    );
    packet.insert("local".to_owned(), VariantValue::Bool(false));
    packet.insert(
        "position".to_owned(),
        VariantValue::Vector3(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
    );
    packet.insert("zone".to_owned(), VariantValue::String("".to_string()));
    packet.insert("zone_owner".to_owned(), VariantValue::Int(-1));

    encode_variant(VariantValue::Dictionary(packet))
}

/// Builds a `handshake` packet. This packet represents a successful P2P connection from the given
/// user.
pub fn build_handshake_packet(user_id: SteamId) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert(
        "type".to_owned(),
        VariantValue::String("handshake".to_string()),
    );
    packet.insert(
        "user_id".to_owned(),
        VariantValue::String(user_id.raw().to_string()),
    );

    encode_variant(VariantValue::Dictionary(packet))
}

/// Builds a `force_disconnect_player` packet. This packet tells clients to mark the supplied user
/// as "jailed". This is used to prevent a user from reconnecting to P2P peers.
pub fn build_force_disconnect_player_packet(user_id: &u64) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert(
        "type".to_owned(),
        VariantValue::String("force_disconnect_player".to_string()),
    );
    packet.insert(
        "user_id".to_owned(),
        VariantValue::String(user_id.to_string()),
    );

    encode_variant(VariantValue::Dictionary(packet))
}

pub fn build_instance_actor_packet(actor: &Actor) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert(
        "type".to_owned(),
        VariantValue::String("instance_actor".to_string()),
    );

    packet.insert("params".to_owned(), actor.clone_to_variant_dict());

    encode_variant(VariantValue::Dictionary(packet))
}

pub fn build_actor_update_packet(actor: &Actor) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert(
        "type".to_owned(),
        VariantValue::String("actor_update".to_string()),
    );

    packet.insert("actor_id".to_owned(), VariantValue::Int(actor.id));
    packet.insert("pos".to_owned(), VariantValue::Vector3(actor.position.clone()));
    packet.insert("rot".to_owned(), VariantValue::Vector3(actor.rotation.clone()));

    encode_variant(VariantValue::Dictionary(packet))
}

pub fn build_actor_action_packet(actor: &Actor, action: &str, params: Array) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert(
        "type".to_owned(),
        VariantValue::String("actor_action".to_string()),
    );

    packet.insert("actor_id".to_owned(), VariantValue::Int(actor.id));
    packet.insert("action".to_owned(), VariantValue::String(action.to_string()));
    packet.insert("params".to_owned(), VariantValue::Array(params));

    encode_variant(VariantValue::Dictionary(packet))
}

pub fn build_actor_request_packet(user_id: SteamId) -> Vec<u8> {
    let mut packet = Dictionary::new();
    packet.insert("type".to_owned(), VariantValue::String("request_actors".to_owned()));
    
    packet.insert("user_id".to_owned(), VariantValue::String(user_id.raw().to_string()));

    encode_variant(VariantValue::Dictionary(packet))
}
