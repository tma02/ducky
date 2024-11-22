use actor::ActorManager;
use peer::PeerManager;
use spawn::SpawnManager;

use crate::server::Server;

pub mod actor;
pub mod peer;
pub mod spawn;

pub struct Game {
    pub actor_manager: ActorManager,
    pub spawn_manager: SpawnManager,
    pub peer_manager: PeerManager,
}

impl Game {
    pub fn new() -> Self {
        Self {
            actor_manager: ActorManager::new(),
            spawn_manager: SpawnManager::new(),
            peer_manager: PeerManager::new(),
        }
    }

    pub fn on_ready(&mut self, server: &mut Server) {
        self.spawn_manager.on_ready(server, &mut self.actor_manager);
    }
    
    pub fn on_update(&mut self, server: &mut Server) {
        self.spawn_manager.on_update(server, &mut self.actor_manager);
        self.peer_manager.on_update(server);
    }
}
