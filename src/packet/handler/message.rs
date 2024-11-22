use steamworks::SteamId;

use crate::{
    command::{handler::resolve_handler, CommandContext},
    game::Game,
    packet::variant::{Dictionary, VariantValue},
    server::Server,
};

const TAG: &str = "message";

pub fn handle(server: &mut Server, game: &mut Game, steam_id: SteamId, packet: Dictionary) {
    let Some(VariantValue::String(message)) = packet.get("message") else {
        return;
    };
    println!(
        "[{}] Received message from {}: {:?}",
        TAG,
        steam_id.raw(),
        message
    );
    let stripped_message = message.replace("%u: ", "");
    if stripped_message.starts_with('!') {
        let command = &stripped_message.split_whitespace().next().unwrap()[1..];
        let args = stripped_message
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>();

        let command_context = CommandContext {
            sender: steam_id,
            command,
            _args: args,
        };
        if let Some(handler) = resolve_handler(&command_context) {
            handler(server, game, command_context);
        }
    }
}
