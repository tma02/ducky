use crate::{
    command::CommandContext,
    game::{actor::ActorType, Game},
    Server,
};

static TAG: &str = "rain";

pub fn handle(server: &mut Server, game: &mut Game, command_ctx: CommandContext) {
    let Some(player_actor) = game.actor_manager.get_player_actor(&command_ctx.sender) else {
        server.send_chat_message(&command_ctx.sender, "Failed. No Player character found.");
        return;
    };

    // TODO: We should actually check if the server host SteamId can create the actor
    if !game
        .actor_manager
        .user_can_create_actor(&command_ctx.sender, true, &ActorType::Raincloud)
    {
        server.send_chat_message(&command_ctx.sender, "Failed. You have too many props!");
        println!(
            "[{TAG}] User cannot create raincloud actor: steam_id = {}",
            command_ctx.sender.raw()
        );
        return;
    }

    let spawn_manager = &mut game.spawn_manager;
    if !spawn_manager.can_spawn_user_actor(&ActorType::Raincloud) {
        let _ = server.send_chat_message(
            &command_ctx.sender,
            "Failed. Someone already spawned a rain cloud.",
        );
        return;
    }
    let mut raincloud_position = player_actor.position.clone();
    raincloud_position.y = 42.0;
    spawn_manager.spawn_user_raincloud(
        server,
        &mut game.actor_manager,
        "main_zone",
        &raincloud_position,
    );

    server.send_chat_message(&command_ctx.sender, "Spawned rain cloud.");
}
