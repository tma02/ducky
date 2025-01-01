use std::{collections::HashSet, sync::mpsc::Sender};

use steamworks::{Client, LobbyId, SendType, SteamId};

use crate::{config::Config, packet::{
    util::{build_message_packet, send_variant_p2p},
    OutgoingP2pPacketRequest, P2pChannel, P2pPacketTarget,
}};

pub struct Server {
    pub steam_client: Client,
    pub sender_p2p_packet: Sender<OutgoingP2pPacketRequest>,
    // TODO: Holding lobby_id here means we can't have multiple lobbies open at once.
    pub lobby_id: Option<LobbyId>,
    // TODO: Holding ban_list here means we can't have per-lobby ban lists.
    /// A list of banned SteamIds as raw u64.
    pub ban_list: HashSet<u64>,
    pub config: Config,
    /// A list of users in the lobby.
    pub users: HashSet<u64>,
}

impl Server {
    pub fn new(
        client: Client,
        sender_p2p_packet: Sender<OutgoingP2pPacketRequest>,
        config: Config,
    ) -> Self {
        Self {
            steam_client: client,
            sender_p2p_packet,
            lobby_id: None,
            ban_list: HashSet::new(),
            config,
            users: HashSet::new(),
        }
    }

    pub fn set_lobby_id(&mut self, lobby_id: LobbyId) {
        self.lobby_id = Some(lobby_id);
    }

    pub fn insert_ban_list(&mut self, steam_id: u64) {
        self.ban_list.insert(steam_id);
    }

    pub fn banned_steam_id(&self, steam_id: &SteamId) -> bool {
        self.ban_list.contains(&steam_id.raw())
    }

    // This is a utility function for sending a packet, does this belong here?
    pub fn send_chat_message(&self, steam_id: &SteamId, message: &str) {
        send_variant_p2p(
            &self.sender_p2p_packet,
            build_message_packet(message),
            P2pPacketTarget::SteamId(steam_id.clone()),
            P2pChannel::GameState,
            SendType::Reliable,
        );
    }
}
