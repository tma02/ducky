use serde::Deserialize;

use crate::random::lobby_code;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_motd")]
    pub motd: String,
    #[serde(default = "default_game_version")]
    pub game_version: String,
    #[serde(default = "default_lobby_code")]
    pub lobby_code: String,
    #[serde(default = "default_max_players")]
    pub max_players: u32,
    #[serde(default = "default_unlisted")]
    pub unlisted: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_talkative: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_quiet: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_grinding: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_chill: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_silly: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_hardcore: bool,
    #[serde(default = "default_tag_generic")]
    pub tag_mature: bool,
    #[serde(default = "default_tag_modded")]
    pub tag_modded: bool,
    #[serde(default = "default_ban_list")]
    pub ban_list: Vec<u64>,
}

impl Config {
    pub fn get_lobby_data_for_bool(value: bool) -> String {
        if value { "1".to_owned() } else { "0".to_owned() }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: default_name(),
            motd: default_motd(),
            game_version: default_game_version(),
            lobby_code: default_lobby_code(),
            max_players: default_max_players(),
            unlisted: default_unlisted(),
            tag_talkative: default_tag_generic(),
            tag_quiet: default_tag_generic(),
            tag_grinding: default_tag_generic(),
            tag_chill: default_tag_generic(),
            tag_silly: default_tag_generic(),
            tag_hardcore: default_tag_generic(),
            tag_mature: default_tag_generic(),
            tag_modded: default_tag_modded(),
            ban_list: default_ban_list(),
        }
    }
}

fn default_name() -> String {
    "A Ducky Server".to_string()
}
fn default_motd() -> String {
    "This lobby is powered by Ducky.\nType !help to see commands.".to_string()
}
fn default_game_version() -> String {
    "1.11".to_string()
}
fn default_lobby_code() -> String {
    lobby_code()
}
fn default_max_players() -> u32 {
    12
}
fn default_unlisted() -> bool {
    false
}
fn default_tag_generic() -> bool {
    false
}
fn default_tag_modded() -> bool {
    true
}
fn default_ban_list() -> Vec<u64> {
    vec![]
}
