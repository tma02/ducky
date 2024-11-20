use crate::{game::Game, server::Server};

use super::CommandContext;

mod help;
mod rain;

/// Returns the handler function for the given command and args.
pub fn resolve_handler(
    command_ctx: &CommandContext,
) -> Option<fn(&mut Server, &mut Game, CommandContext)> {
    // TODO: Map these with some registry system? Would be very useful for user-defined commands,
    //  and would make it easier to create and manage new commands later.
    match command_ctx.command {
        "help" | "commands" => Some(help::handle),
        "rain" => Some(rain::handle),
        _ => None,
    }
}
