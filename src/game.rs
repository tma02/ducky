use actor::ActorManager;
use spawn::SpawnManager;

use crate::server::Server;

pub mod actor;
pub mod spawn;

pub struct Game {
    pub actor_manager: ActorManager,
    pub spawn_manager: SpawnManager,
}

impl Game {
    pub fn new() -> Self {
        Self {
            actor_manager: ActorManager::new(),
            spawn_manager: SpawnManager::new(),
        }
    }

    pub fn on_ready(&mut self, server: &mut Server) {
        self.spawn_manager.on_ready(server, &mut self.actor_manager);
    }
    
    pub fn on_update(&mut self, server: &mut Server) {
        self.spawn_manager.on_update(server, &mut self.actor_manager);
    }
}
