use std::time::Instant;

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
        let next_raincloud_instant = spawn_manager.next_user_spawn_instant(&ActorType::Raincloud);
        if let Some(next_raincloud_instant) = next_raincloud_instant {
            let _ = server.send_chat_message(
                &command_ctx.sender,
                format!(
                    "Someone already spawned a rain cloud. Please wait {}.",
                    format_timeout_from_now(next_raincloud_instant)
                )
                .as_str(),
            );
        } else {
            let _ = server.send_chat_message(
                &command_ctx.sender,
                "Someone already spawned a rain cloud. Please wait for it to despawn.",
            );
        }
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

fn format_timeout_from_now(instant: &Instant) -> String {
    let duration = instant.duration_since(Instant::now());
    let secs = duration.as_secs();

    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
    }
}
