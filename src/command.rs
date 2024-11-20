use steamworks::SteamId;

use crate::packet::variant::Dictionary;

pub mod handler;

pub struct CommandContext<'a> {
    pub sender: SteamId,
    pub packet: Dictionary,
    pub command: &'a str,
    pub args: Vec<&'a str>,
}
