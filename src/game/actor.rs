use std::{collections::HashMap, sync::mpsc::Sender};

use steamworks::{SendType, SteamId};

use crate::packet::{
    util::{build_actor_action_packet, build_actor_update_packet, build_instance_actor_packet},
    variant::{Dictionary, VariantValue, Vector3},
    OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
};

const MAX_ACTORS_PER_PLAYER: usize = 32;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActorType {
    Unknown,
    Player,
    FishSpawn,
    FishSpawnAlien,
    Raincloud,
    RaincloudTiny,
    AquaFish,
    MetalSpawn,
    AmbientBird,
    VoidPortal,
    Picnic,
    Canvas,
    Bush,
    Rock,
    FishTrap,
    FishTrapOcean,
    IslandTiny,
    IslandMed,
    IslandBig,
    Boombox,
    Well,
    Campfire,
    Chair,
    Table,
    TherapistChair,
    Toilet,
    Whoopie,
    Beer,
    Greenscreen,
    PortableBait,
}

impl ActorType {
    /// Returns if this ActorType can only be created by the host.
    pub fn is_create_by_host_only(&self) -> bool {
        match self {
            ActorType::FishSpawn => true,
            ActorType::FishSpawnAlien => true,
            ActorType::Raincloud => true,
            ActorType::MetalSpawn => true,
            ActorType::AmbientBird => true,
            ActorType::VoidPortal => true,
            _ => false,
        }
    }
}

impl From<&str> for ActorType {
    fn from(value: &str) -> Self {
        match value {
            "player" => ActorType::Player,
            "fish_spawn" => ActorType::FishSpawn,
            "fish_spawn_alien" => ActorType::FishSpawnAlien,
            "raincloud" => ActorType::Raincloud,
            "raincloud_tiny" => ActorType::RaincloudTiny,
            "aqua_fish" => ActorType::AquaFish,
            "metal_spawn" => ActorType::MetalSpawn,
            "ambient_bird" => ActorType::AmbientBird,
            "void_portal" => ActorType::VoidPortal,
            "picnic" => ActorType::Picnic,
            "canvas" => ActorType::Canvas,
            "bush" => ActorType::Bush,
            "rock" => ActorType::Rock,
            "fish_trap" => ActorType::FishTrap,
            "fish_trap_ocean" => ActorType::FishTrapOcean,
            "island_tiny" => ActorType::IslandTiny,
            "island_med" => ActorType::IslandMed,
            "island_big" => ActorType::IslandBig,
            "boombox" => ActorType::Boombox,
            "well" => ActorType::Well,
            "campfire" => ActorType::Campfire,
            "chair" => ActorType::Chair,
            "table" => ActorType::Table,
            "therapist_chair" => ActorType::TherapistChair,
            "toilet" => ActorType::Toilet,
            "whoopie" => ActorType::Whoopie,
            "beer" => ActorType::Beer,
            "greenscreen" => ActorType::Greenscreen,
            "portable_bait" => ActorType::PortableBait,
            _ => ActorType::Unknown,
        }
    }
}

impl From<ActorType> for &'static str {
    fn from(value: ActorType) -> Self {
        match value {
            ActorType::Unknown => "unknown",
            ActorType::Player => "player",
            ActorType::FishSpawn => "fish_spawn",
            ActorType::FishSpawnAlien => "fish_spawn_alien",
            ActorType::Raincloud => "raincloud",
            ActorType::RaincloudTiny => "raincloud_tiny",
            ActorType::AquaFish => "aqua_fish",
            ActorType::MetalSpawn => "metal_spawn",
            ActorType::AmbientBird => "ambient_bird",
            ActorType::VoidPortal => "void_portal",
            ActorType::Picnic => "picnic",
            ActorType::Canvas => "canvas",
            ActorType::Bush => "bush",
            ActorType::Rock => "rock",
            ActorType::FishTrap => "fish_trap",
            ActorType::FishTrapOcean => "fish_trap_ocean",
            ActorType::IslandTiny => "island_tiny",
            ActorType::IslandMed => "island_med",
            ActorType::IslandBig => "island_big",
            ActorType::Boombox => "boombox",
            ActorType::Well => "well",
            ActorType::Campfire => "campfire",
            ActorType::Chair => "chair",
            ActorType::Table => "table",
            ActorType::TherapistChair => "therapist_chair",
            ActorType::Toilet => "toilet",
            ActorType::Whoopie => "whoopie",
            ActorType::Beer => "beer",
            ActorType::Greenscreen => "greenscreen",
            ActorType::PortableBait => "portable_bait",
        }
    }
}

impl From<ActorType> for String {
    fn from(value: ActorType) -> Self {
        <&str>::from(value).to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    pub id: i64,
    pub creator_id: SteamId,
    pub actor_type: ActorType,
    pub zone: String,
    pub zone_owner: i64,
    pub position: Vector3,
    pub rotation: Vector3,
}

impl Actor {
    /// Creates a new owned `VariantValue::Dictionary` which matches the `params` dict used in
    /// the `instance_actor` packet.
    pub fn clone_to_variant_dict(&self) -> VariantValue {
        let mut params = Dictionary::new();
        params.insert("actor_id".to_owned(), VariantValue::Int(self.id));
        params.insert(
            "creator_id".to_owned(),
            VariantValue::Int(self.creator_id.raw() as i64),
        );
        params.insert(
            "actor_type".to_owned(),
            VariantValue::String(String::from(self.actor_type.clone())),
        );
        params.insert("zone".to_owned(), VariantValue::String(self.zone.clone()));
        params.insert("zone_owner".to_owned(), VariantValue::Int(self.zone_owner));
        params.insert(
            "at".to_owned(),
            VariantValue::Vector3(self.position.clone()),
        );
        params.insert(
            "rot".to_owned(),
            VariantValue::Vector3(self.rotation.clone()),
        );

        VariantValue::Dictionary(params)
    }
}

pub struct ActorManager {
    actors_by_id: HashMap<i64, Actor>,
    actor_ids_by_creator: HashMap<SteamId, Vec<i64>>,
    player_actor_ids_by_creator: HashMap<SteamId, i64>,
}

impl ActorManager {
    pub fn new() -> Self {
        Self {
            actors_by_id: HashMap::new(),
            actor_ids_by_creator: HashMap::new(),
            player_actor_ids_by_creator: HashMap::new(),
        }
    }

    /// Inserts an actor into the ActorManager. This does not perform any checks or network sync.
    pub fn insert_actor(&mut self, actor: Actor) {
        self.actors_by_id.insert(actor.id.clone(), actor.clone());
        self.actor_ids_by_creator
            .entry(actor.creator_id.clone())
            .or_default()
            .push(actor.id.clone());
        if actor.actor_type == ActorType::Player {
            self.player_actor_ids_by_creator
                .insert(actor.creator_id.clone(), actor.id.clone());
        }
    }

    /// Removes an actor from the ActorManager. This does not perform any checks or network sync.
    pub fn remove_actor(&mut self, id: &i64) {
        if let Some(actor) = self.actors_by_id.remove(id) {
            if let Some(actor_ids) = self.actor_ids_by_creator.get_mut(&actor.creator_id) {
                actor_ids.retain(|x| x != id);
            }
            if let Some(player_actor_id) = self.player_actor_ids_by_creator.get(&actor.creator_id) {
                if player_actor_id == id {
                    self.player_actor_ids_by_creator.remove(&actor.creator_id);
                }
            }
        }
    }

    /// Spawns an actor. This will check if the actor is valid and then broadcast the
    /// `instance_actor` packet.
    pub fn spawn_host_actor(
        &mut self,
        sender_p2p_packet: &Sender<OutgoingP2pPacketRequest>,
        creator_id: &SteamId,
        actor: Actor,
    ) -> bool {
        if !self.user_can_create_actor(creator_id, true, &actor.actor_type) {
            return false;
        }

        self.insert_actor(actor.clone());
        let _ = sender_p2p_packet.send(OutgoingP2pPacketRequest {
            data: build_instance_actor_packet(&actor),
            target: P2pPacketTarget::All,
            channel: P2pChannel::GameState,
            send_type: SendType::Reliable,
        });
        // Without `actor_update` the actor tends to stay at the world origin.
        let _ = sender_p2p_packet.send(OutgoingP2pPacketRequest {
            data: build_actor_update_packet(&actor),
            target: P2pPacketTarget::All,
            channel: P2pChannel::ActorUpdate,
            send_type: SendType::Reliable,
        });

        true
    }

    /// Despawns an actor. This will broadcast the `actor_action` with `action` set to `queue_free`
    /// and remove the actor from the ActorManager.
    pub fn despawn_host_actor(
        &mut self,
        sender_p2p_packet: &Sender<OutgoingP2pPacketRequest>,
        actor_id: &i64,
    ) -> bool {
        let Some(actor) = self.actors_by_id.get(actor_id) else {
            return false;
        };

        let _ = sender_p2p_packet.send(OutgoingP2pPacketRequest {
            data: build_actor_action_packet(&actor, "queue_free", vec![]),
            target: P2pPacketTarget::All,
            channel: P2pChannel::ActorAction,
            send_type: SendType::Reliable,
        });
        self.remove_actor(actor_id);

        true
    }

    /// Gets the player actor of the given SteamId.
    pub fn get_player_actor(&self, steam_id: &SteamId) -> Option<&Actor> {
        self.player_actor_ids_by_creator
            .get(steam_id)
            .and_then(|id| self.actors_by_id.get(id))
    }

    /// Gets the player actor of the user with the given SteamId.
    pub fn get_player_actor_mut(&mut self, creator_id: &SteamId) -> Option<&mut Actor> {
        self.player_actor_ids_by_creator
            .get(creator_id)
            .and_then(|id| self.actors_by_id.get_mut(id))
    }

    /// Gets the actor with the given actor ID.
    pub fn get_actor(&self, id: &i64) -> Option<&Actor> {
        self.actors_by_id.get(id)
    }

    /// Gets the actor with the given actor ID.
    pub fn get_actor_mut(&mut self, id: &i64) -> Option<&mut Actor> {
        self.actors_by_id.get_mut(id)
    }

    /// Updates an actor's position, rotation, and zone.
    pub fn update_host_actor(
        &mut self,
        sender_p2p_packet: &Sender<OutgoingP2pPacketRequest>,
        id: &i64,
        position: &Vector3,
        rotation: &Vector3,
    ) -> Option<&Actor> {
        let Some(actor) = self.actors_by_id.get_mut(id) else {
            return None;
        };

        actor.position.x = position.x;
        actor.position.y = position.y;
        actor.position.z = position.z;
        actor.rotation.x = rotation.x;
        actor.rotation.y = rotation.y;
        actor.rotation.z = rotation.z;

        let _ = sender_p2p_packet.send(OutgoingP2pPacketRequest {
            data: build_actor_update_packet(actor),
            target: P2pPacketTarget::All,
            channel: P2pChannel::ActorUpdate,
            send_type: SendType::Reliable,
        });

        Some(actor)
    }

    pub fn set_host_actor_zone(&mut self, id: &i64, zone: String) -> Option<&Actor> {
        if let Some(actor) = self.actors_by_id.get_mut(id) {
            actor.zone = zone;
            todo!(); // TODO: Broadcast actor update

            return Some(actor);
        }

        None
    }

    /// Gets all actors created by the given SteamId.
    pub fn get_actors_by_creator(&self, creator_id: &SteamId) -> Option<Vec<&Actor>> {
        self.actor_ids_by_creator.get(creator_id).map(|ids| {
            ids.iter()
                .map(|id| self.actors_by_id.get(id).unwrap())
                .collect()
        })
    }

    /// Gets all actors in the given zone.
    pub fn get_actors_by_zone(&self, zone: &str) -> Vec<&Actor> {
        self.actors_by_id
            .values()
            .filter(|actor| actor.zone == *zone)
            .collect()
    }

    /// Gets all actors of the given actor type.
    pub fn get_actors_by_type(&self, actor_type: &ActorType) -> Vec<&Actor> {
        self.actors_by_id
            .values()
            .filter(|actor| actor.actor_type == *actor_type)
            .collect()
    }

    /// Checks if a user can create an actor of the given type.
    pub fn user_can_create_actor(
        &self,
        creator_id: &SteamId,
        creator_is_host: bool,
        actor_type: &ActorType,
    ) -> bool {
        if *actor_type == ActorType::Player {
            return self.player_actor_ids_by_creator.get(creator_id).is_none();
        }

        if actor_type.is_create_by_host_only() && !creator_is_host {
            return false;
        }

        self.get_actors_by_creator(creator_id)
            .map(|v| v.len() < MAX_ACTORS_PER_PLAYER)
            // If the creator does not have any actors, they can create one.
            .unwrap_or(true)
    }
}
