use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use steamworks::{SendType, SteamId};

use crate::{
    packet::{
        util::{build_actor_request_packet, send_variant_p2p},
        P2pChannel, P2pPacketTarget,
    },
    server::Server,
};

static TAG: &str = "game::peer";
static TIMEOUT: Duration = Duration::from_secs(5);

pub struct PeerManager {
    steam_ids_need_actor_update: HashSet<SteamId>,
    last_actor_update_request: Instant,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            steam_ids_need_actor_update: HashSet::new(),
            last_actor_update_request: Instant::now(),
        }
    }

    pub fn add_peer_need_update(&mut self, steam_id: SteamId) {
        self.steam_ids_need_actor_update.insert(steam_id);
    }

    pub fn on_update(&mut self, server: &Server) {
        if self.last_actor_update_request.elapsed() > TIMEOUT {
            self.request_actor_update(server);
        }
    }

    fn request_actor_update(&mut self, server: &Server) {
        let steam_ids_to_update = std::mem::take(&mut self.steam_ids_need_actor_update);
        for steam_id in steam_ids_to_update {
            println!(
                "[{TAG}] Requesting actor update: steam_id = {}",
                steam_id.raw()
            );
            send_variant_p2p(
                &server.sender_p2p_packet,
                build_actor_request_packet(server.steam_client.user().steam_id()),
                P2pPacketTarget::SteamId(steam_id),
                P2pChannel::GameState,
                SendType::Reliable,
            );
        }

        self.last_actor_update_request = Instant::now();
    }
}
