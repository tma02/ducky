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
    #[serde(default = "default_code_only")]
    pub code_only: bool,
    #[serde(default = "default_adult_only")]
    pub adult_only: bool,
    #[serde(default = "default_ban_list")]
    pub ban_list: Vec<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: default_name(),
            motd: default_motd(),
            game_version: default_game_version(),
            lobby_code: default_lobby_code(),
            max_players: default_max_players(),
            code_only: default_code_only(),
            adult_only: default_adult_only(),
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
fn default_code_only() -> bool {
    true
}
fn default_adult_only() -> bool {
    false
}
fn default_ban_list() -> Vec<u64> {
    vec![]
}
