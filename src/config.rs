use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub game_version: String,
    pub max_players: u32,
    pub code_only: bool,
    pub ban_list: Vec<u64>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "A Ducky Server".to_owned(),
            game_version: "1.1".to_owned(),
            max_players: 25,
            code_only: true,
            ban_list: vec![],
        }
    }
}
