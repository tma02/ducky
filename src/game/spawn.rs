use std::{
    collections::HashMap,
    fs,
    sync::{mpsc::Sender, LazyLock},
    time::{Duration, Instant},
};

use rand::seq::SliceRandom;

use crate::{
    packet::{variant::Vector3, OutgoingP2pPacketRequest},
    random::{godot_rand_range, godot_randf, godot_randi},
    server::Server,
};

use super::actor::{Actor, ActorManager, ActorType};

static TAG: &str = "game::spawn";
// TODO: make these configurable
static SPAWN_LIFETIMES: LazyLock<HashMap<ActorType, Duration>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    // One physics tick is 1/60s
    map.insert(ActorType::Raincloud, Duration::from_secs(32500 / 60));
    map.insert(ActorType::FishSpawn, Duration::from_secs(4800 / 60));
    map.insert(ActorType::FishSpawnAlien, Duration::from_secs(14400 / 60));
    map.insert(ActorType::MetalSpawn, Duration::from_secs(10000 / 60));
    map.insert(ActorType::VoidPortal, Duration::from_secs(36000 / 60));
    map.insert(ActorType::AmbientBird, Duration::MAX);

    map
});
// TODO: make these configurable
static SPAWN_COUNT_LIMITS: LazyLock<HashMap<ActorType, usize>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(ActorType::Raincloud, 2);
    map.insert(ActorType::FishSpawn, 16);
    map.insert(ActorType::FishSpawnAlien, 4);
    map.insert(ActorType::MetalSpawn, 8);
    map.insert(ActorType::VoidPortal, 1);
    map.insert(ActorType::AmbientBird, 9);

    map
});

pub struct SpawnManager {
    /// Spawns which are built into the game.
    game_spawns: HashMap<ActorType, Vec<i64>>,
    /// Spawns initiated by user commands (these are still spawned by the host SteamId).
    user_spawns: HashMap<ActorType, Vec<i64>>,
    /// A map of a spawn's actor ID to the Instant they should despawn.
    spawn_timeouts: HashMap<i64, Instant>,
    spawn_points: HashMap<String, Vec<Vector3>>,
    next_host_spawn: Instant,
    next_ambient_spawn: Instant,
    next_metal_spawn: Instant,
    alien_cooldown: u64,
    rain_chance: f64,
}

// TODO: make these configurable
fn next_host_spawn() -> Instant {
    Instant::now() + Duration::from_secs(10)
}

fn next_ambient_spawn() -> Instant {
    Instant::now() + Duration::from_secs(10)
}

fn next_metal_spawn() -> Instant {
    Instant::now() + Duration::from_secs(20)
}

impl SpawnManager {
    pub fn new() -> Self {
        let spawn_points: HashMap<String, Vec<Vector3>> =
            match fs::read_to_string("./data/spawn_points.json") {
                Ok(spawn_points) => serde_json::from_str(&spawn_points).unwrap_or(HashMap::new()),
                _ => HashMap::new(),
            };

        SpawnManager {
            game_spawns: HashMap::new(),
            user_spawns: HashMap::new(),
            spawn_timeouts: HashMap::new(),
            spawn_points,
            next_host_spawn: next_host_spawn(),
            next_ambient_spawn: next_ambient_spawn(),
            next_metal_spawn: next_metal_spawn(),
            alien_cooldown: 16, // default
            rain_chance: godot_rand_range(0.0, 0.2),
        }
    }

    pub fn on_ready(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        for _ in 0..4 {
            self.spawn_game_metal_spawn(server, actor_manager);
        }
    }

    pub fn on_update(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if Instant::now() >= self.next_host_spawn {
            self.next_host_spawn = next_host_spawn();
            self.spawn_random_game_actor(server, actor_manager);
        }
        if Instant::now() >= self.next_ambient_spawn {
            self.next_ambient_spawn = next_ambient_spawn();
            self.spawn_game_bird(server, actor_manager);
        }
        if Instant::now() >= self.next_metal_spawn {
            self.next_metal_spawn = next_metal_spawn();
            self.spawn_game_metal_spawn(server, actor_manager);
        }

        // Process expired actors.
        for actor_id in self.get_actors_need_despawn(Instant::now()) {
            println!("[{TAG}] Despawning expired actor: actor_id = {}", actor_id);
            self.despawn_actor(&server.sender_p2p_packet, actor_manager, &actor_id);
        }
    }

    fn spawn_random_game_actor(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        let mut actor_type = if godot_randi() % 2 == 0 {
            ActorType::FishSpawn
        } else {
            ActorType::Unknown
        };

        if let Some(alien_cooldown) = self.alien_cooldown.checked_sub(1) {
            self.alien_cooldown = alien_cooldown;
        }
        if godot_randf() < 0.01
            && godot_randf() < 0.4
            && actor_manager
                .get_actors_by_type(&ActorType::FishSpawnAlien)
                .len()
                == 0
            && self.alien_cooldown <= 0
        {
            actor_type = ActorType::FishSpawnAlien;
            self.alien_cooldown = 16;
        }

        if godot_randf() < self.rain_chance && godot_randf() < 0.12 {
            actor_type = ActorType::Raincloud;
            self.rain_chance = 0.0;
        } else {
            if godot_randf() < 0.75 {
                self.rain_chance += 0.001;
            }
        }

        if godot_randf() < 0.01 && godot_randf() < 0.25 {
            actor_type = ActorType::VoidPortal;
        }

        match actor_type {
            ActorType::FishSpawn => self.spawn_game_fish(server, actor_manager),
            ActorType::FishSpawnAlien => self.spawn_game_fish_alien(server, actor_manager),
            ActorType::Raincloud => self.spawn_game_raincloud(server, actor_manager),
            ActorType::VoidPortal => self.spawn_game_void_portal(server, actor_manager),
            _ => (),
        };
    }

    /// Despawns an actor and broadcasts the despawn packet to all clients.
    fn despawn_actor(
        &mut self,
        sender: &Sender<OutgoingP2pPacketRequest>,
        actor_manager: &mut ActorManager,
        id: &i64,
    ) {
        println!("[{TAG}] Despawning actor: actor_id = {id}");

        self.game_spawns.iter_mut().for_each(|(_, spawns)| {
            spawns.retain(|spawn_id| spawn_id != id);
        });
        self.spawn_timeouts.remove(id);
        actor_manager.despawn_host_actor(sender, id);
    }

    /// Spawns an actor triggered by the game logic. This will broadcast both `instance_actor` and
    /// initial `actor_update` packets.
    fn spawn_game_actor(
        &mut self,
        context: &mut Server,
        actor_manager: &mut ActorManager,
        actor: Actor,
    ) {
        println!(
            "[{TAG}] Spawning game actor with type: actor_type = {:?}",
            actor.actor_type
        );
        let id = actor.id.clone();
        let actor_type = actor.actor_type.clone();

        actor_manager.spawn_host_actor(
            &context.sender_p2p_packet,
            &context.steam_client.user().steam_id(),
            actor,
        );
        self.game_spawns
            .entry(ActorType::Raincloud)
            .or_insert(vec![])
            .push(id);

        let spawn_lifetime = *SPAWN_LIFETIMES.get(&actor_type).unwrap_or(&Duration::MAX);
        if spawn_lifetime < Duration::MAX {
            Instant::now()
                .checked_add(spawn_lifetime)
                .map(|i| self.spawn_timeouts.insert(id, i));
        }
    }

    /// Spawns a raincloud using game logic. This will broadcast the spawn to all clients.
    fn spawn_game_raincloud(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if !self.can_spawn_game_actor(&ActorType::Raincloud) {
            println!("[{TAG}] Failed spawn_game_raincloud: actor count limit reached");
            return;
        }

        let raincloud = Actor {
            id: godot_randi(),
            creator_id: server.steam_client.user().steam_id(),
            actor_type: ActorType::Raincloud,
            zone: "main_zone".to_owned(),
            zone_owner: -1,
            position: Vector3 {
                x: godot_rand_range(-100.0, 150.0),
                y: 42.0,
                z: godot_rand_range(-150.0, 100.0),
            },
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.spawn_game_actor(server, actor_manager, raincloud);
    }

    fn spawn_game_metal_spawn(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if !self.can_spawn_game_actor(&ActorType::MetalSpawn) {
            println!("[{TAG}] Failed spawn_game_metal_spawn: actor count limit reached");
            return;
        }

        let use_shoreline = godot_randf() < 0.15;
        let position = if use_shoreline {
            self.random_spawn_point("shoreline_point")
        } else {
            self.random_spawn_point("trash_point")
        };
        let Some(position) = position else {
            println!("[{TAG}] Failed spawn_game_metal_spawn: no spawn point found");
            return;
        };

        let mut position = position.clone();
        position.x += godot_rand_range(-0.5, 0.5);
        position.z += godot_rand_range(-0.5, 0.5);

        let metal_spawn = Actor {
            id: godot_randi(),
            creator_id: server.steam_client.user().steam_id(),
            actor_type: ActorType::MetalSpawn,
            zone: "main_zone".to_owned(),
            zone_owner: -1,
            position,
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.spawn_game_actor(server, actor_manager, metal_spawn);
    }

    fn spawn_game_fish(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if !self.can_spawn_game_actor(&ActorType::FishSpawn) {
            println!("[{TAG}] Failed spawn_game_fish: actor count limit reached");
            return;
        }

        let position = self.random_spawn_point("fish_spawn");
        let Some(position) = position else {
            println!("[{TAG}] Failed spawn_game_fish: no spawn point found");
            return;
        };

        let fish_spawn = Actor {
            id: godot_randi(),
            creator_id: server.steam_client.user().steam_id(),
            actor_type: ActorType::FishSpawn,
            zone: "main_zone".to_owned(),
            zone_owner: -1,
            position: position.clone(),
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.spawn_game_actor(server, actor_manager, fish_spawn);
    }

    fn spawn_game_fish_alien(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if !self.can_spawn_game_actor(&ActorType::FishSpawnAlien) {
            println!("[{TAG}] Failed spawn_game_fish_alien: actor count limit reached");
            return;
        }

        let position = self.random_spawn_point("fish_spawn");
        let Some(position) = position else {
            println!("[{TAG}] Failed spawn_game_fish_alien: no spawn point found");
            return;
        };

        let fish_spawn_alien = Actor {
            id: godot_randi(),
            creator_id: server.steam_client.user().steam_id(),
            actor_type: ActorType::FishSpawnAlien,
            zone: "main_zone".to_owned(),
            zone_owner: -1,
            position: position.clone(),
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.spawn_game_actor(server, actor_manager, fish_spawn_alien);
    }

    fn spawn_game_void_portal(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if !self.can_spawn_game_actor(&ActorType::VoidPortal) {
            println!("[{TAG}] Failed spawn_game_void_portal: actor count limit reached");
            return;
        }

        let position = self.random_spawn_point("hidden_spot");
        let Some(position) = position else {
            println!("[{TAG}] Failed spawn_game_void_portal: no spawn point found");
            return;
        };

        let mut position = position.clone();
        position.x += godot_rand_range(-0.5, 0.5);
        position.z += godot_rand_range(-0.5, 0.5);

        let void_portal = Actor {
            id: godot_randi(),
            creator_id: server.steam_client.user().steam_id(),
            actor_type: ActorType::VoidPortal,
            zone: "main_zone".to_owned(),
            zone_owner: -1,
            position: position.clone(),
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.spawn_game_actor(server, actor_manager, void_portal);
    }

    fn spawn_game_bird(&mut self, server: &mut Server, actor_manager: &mut ActorManager) {
        if !self.can_spawn_game_actor(&ActorType::AmbientBird) {
            println!("[{TAG}] Failed spawn_game_bird: actor count limit reached");
            return;
        }

        for _ in 0..(godot_randi() % 3 + 1) {
            let position = self.random_spawn_point("trash_point");
            let Some(position) = position else {
                println!("[{TAG}] Failed spawn_game_bird: no spawn point found");
                return;
            };

            let mut position = position.clone();
            position.x += godot_rand_range(-2.5, 2.5);
            position.z += godot_rand_range(-2.5, 2.5);

            let bird = Actor {
                id: godot_randi(),
                creator_id: server.steam_client.user().steam_id(),
                actor_type: ActorType::AmbientBird,
                zone: "main_zone".to_owned(),
                zone_owner: -1,
                position: position.clone(),
                rotation: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            };

            self.spawn_game_actor(server, actor_manager, bird);
        }
    }

    /// Returns if the current game state permits spawning the given actor type.
    fn can_spawn_game_actor(&self, actor_type: &ActorType) -> bool {
        self.game_spawns
            .get(actor_type)
            .map(|v| v.len())
            .unwrap_or(0)
            < *SPAWN_COUNT_LIMITS.get(actor_type).unwrap_or(&usize::MAX)
    }

    /// Returns a list of actor IDs that need to be despawned.
    pub fn get_actors_need_despawn(&self, now: Instant) -> Vec<i64> {
        let mut actors_need_despawn = vec![];
        for (id, spawn_timeout) in &self.spawn_timeouts {
            if now > *spawn_timeout {
                actors_need_despawn.push(*id);
            }
        }

        actors_need_despawn
    }

    fn spawn_user_actor(
        &mut self,
        server: &mut Server,
        actor_manager: &mut ActorManager,
        actor: Actor,
    ) {
        println!(
            "[{TAG}] Spawning user actor with type: actor_type = {:?}",
            actor.actor_type
        );

        let id = actor.id.clone();
        let actor_type = actor.actor_type.clone();

        actor_manager.spawn_host_actor(
            &server.sender_p2p_packet,
            &server.steam_client.user().steam_id(),
            actor,
        );
        self.user_spawns
            .entry(ActorType::Raincloud)
            .or_insert(vec![])
            .push(id);

        let spawn_lifetime = *SPAWN_LIFETIMES.get(&actor_type).unwrap_or(&Duration::MAX);
        if spawn_lifetime < Duration::MAX {
            Instant::now()
                .checked_add(spawn_lifetime)
                .map(|i| self.spawn_timeouts.insert(id, i));
        }
    }

    /// Spawns a user triggered raincloud. This will broadcast the spawn to all clients.
    pub fn spawn_user_raincloud(
        &mut self,
        server: &mut Server,
        actor_manager: &mut ActorManager,
        zone: &str,
        position: &Vector3,
    ) {
        let raincloud = Actor {
            id: godot_randi(),
            creator_id: server.steam_client.user().steam_id(),
            actor_type: ActorType::Raincloud,
            zone: zone.to_owned(),
            zone_owner: -1,
            position: position.clone(),
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.spawn_user_actor(server, actor_manager, raincloud);
    }

    pub fn can_spawn_user_actor(&self, actor_type: &ActorType) -> bool {
        match actor_type {
            &ActorType::Raincloud => {
                self.user_spawns
                    .get(&ActorType::Raincloud)
                    .map(|v| v.len())
                    .unwrap_or(0)
                    == 0
            }
            _ => false,
        }
    }

    pub fn random_spawn_point(&self, group: &str) -> Option<&Vector3> {
        self.spawn_points
            .get(group)?
            .choose(&mut rand::thread_rng())
    }
}
