use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub motd: String,
    pub game_version: String,
    pub max_players: u32,
    pub code_only: bool,
    pub ban_list: Vec<u64>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "A Ducky Server".to_string(),
            motd: "This lobby is powered by Ducky.\nType !help to see commands.".to_string(),
            game_version: "1.1".to_string(),
            max_players: 25,
            code_only: true,
            ban_list: vec![],
        }
    }
}
