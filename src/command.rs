use steamworks::SteamId;

pub mod handler;

pub struct CommandContext<'a> {
    pub sender: SteamId,
    pub command: &'a str,
    pub args: Vec<&'a str>,
}
