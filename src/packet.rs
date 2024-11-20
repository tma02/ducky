use std::io::Read;

use decode::decode_variant;
use flate2::{bufread::GzEncoder, read::GzDecoder, Compression};
use handler::resolve_handler;
use steamworks::{SendType, SteamId};
use variant::VariantValue;

use crate::{game::Game, Server};

pub mod decode;
pub mod encode;
pub mod handler;
pub mod util;
pub mod variant;

pub enum P2pPacketTarget {
    /// A specific Steam user.
    SteamId(SteamId),
    /// All lobby members. This includes those who are on the kick or ban list.
    All,
    /// All lobby members that are not the sender or host, and not on the kick or ban list. The
    /// value is the sender's SteamId (which will not receive the message).
    Peers(SteamId),
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum P2pChannel {
    ActorUpdate = 0,
    ActorAction = 1,
    GameState = 2,
    Chalk = 3,
    Guitar = 4,
    ActorAnimation = 5,
    Speech = 6,
}

impl P2pChannel {
    pub const VALUES: [Self; 7] = [
        Self::ActorUpdate,
        Self::ActorAction,
        Self::GameState,
        Self::Chalk,
        Self::Guitar,
        Self::ActorAnimation,
        Self::Speech,
    ];
}

pub struct OutgoingP2pPacketRequest {
    pub data: Vec<u8>,
    pub target: P2pPacketTarget,
    pub channel: P2pChannel,
    pub send_type: SendType,
}

pub fn on_receive_packet(
    server: &mut Server,
    game: &mut Game,
    buffer_vec: Vec<u8>,
    remote: SteamId,
) {
    // TODO: check if sender is banned
    let mut d: GzDecoder<&[u8]> = GzDecoder::new(buffer_vec.as_slice());
    let mut decompressed_buf: Vec<u8> = vec![];
    d.read_to_end(&mut decompressed_buf).unwrap();
    let var = decode_variant(&decompressed_buf);
    if let Ok(VariantValue::Dictionary(dict)) = var {
        if let Some(handler) = resolve_handler(&dict) {
            //println!("Handling: {:?}", dict.get("type").unwrap());
            handler(server, game, remote, dict);
        } else {
            //println!("Unknown type for packet: root = {:?}", dict);
        }
    } else {
        println!("Ignoring decode error for: buf = {:?}", decompressed_buf);
    }
}

pub fn on_send_packet(server: &Server, outgoing: OutgoingP2pPacketRequest) {
    let channel_i32 = outgoing.channel as i32;
    let mut e: GzEncoder<&[u8]> = GzEncoder::new(outgoing.data.as_slice(), Compression::fast());
    let mut buffer = Vec::new();
    e.read_to_end(&mut buffer).unwrap();

    if let P2pPacketTarget::SteamId(steam_id) = outgoing.target {
        server.steam_client.networking().send_p2p_packet_on_channel(
            steam_id,
            outgoing.send_type.clone(),
            &buffer,
            channel_i32,
        );
    } else if let Some(lobby_id) = server.lobby_id {
        // TODO: Maybe some way to consolidate this? The only difference between these two blocks is
        //  the 2nd one also ignores a specific SteamId.
        if let P2pPacketTarget::All = outgoing.target {
            for steam_id in server.steam_client.matchmaking().lobby_members(lobby_id) {
                if steam_id == server.steam_client.user().steam_id() {
                    continue;
                }
                server.steam_client.networking().send_p2p_packet_on_channel(
                    steam_id,
                    outgoing.send_type.clone(),
                    &buffer,
                    channel_i32,
                );
            }
        } else if let P2pPacketTarget::Peers(ignore_steam_id) = outgoing.target {
            for steam_id in server.steam_client.matchmaking().lobby_members(lobby_id) {
                if steam_id == server.steam_client.user().steam_id() || steam_id == ignore_steam_id {
                    continue;
                }
                server.steam_client.networking().send_p2p_packet_on_channel(
                    steam_id,
                    outgoing.send_type.clone(),
                    &buffer,
                    channel_i32,
                );
            }
        }
    }
}
