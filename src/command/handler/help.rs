use crate::{command::CommandContext, game::Game, Server};

pub fn handle(server: &mut Server, _game: &mut Game, command_ctx: CommandContext) {
    if let Err(err) =
    server.send_chat_message(&command_ctx.sender, "Available commands: !help, !rain")
    {
        println!("Error sending message: {:?}", err);
    }
}
