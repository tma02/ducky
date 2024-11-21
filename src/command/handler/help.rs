use crate::{command::CommandContext, game::Game, Server};

pub fn handle(server: &mut Server, _game: &mut Game, command_ctx: CommandContext) {
    server.send_chat_message(&command_ctx.sender, "Available commands: !help, !rain")
}
