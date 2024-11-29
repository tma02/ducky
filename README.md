# Ducky

Ducky is an experimental "dedicated server" for WEBFISHING written in Rust. It is an emulation of
the networking and game logic from the perspective of a WEBFISHING lobby host.

## Getting started

### Running

> [!NOTE]
> Ducky requires a running Steam client signed-in with an account who owns WEBFISHING. You cannot
> use the same Steam account for both Ducky and joining the hosted lobby. This is because WEBFISHING
> lobbies can only be created through Steam's matchmaking system.

Running Ducky may, just like all games using Steam's P2P networking, expose your public IP address
to anyone wishing to connect. If this concerns you, check out this Steam Support article under the
heading "What about games that use Steam Networking APIs?":
https://help.steampowered.com/en/faqs/view/1433-AD20-F11D-B71E

```bash
cargo run release
```

#### Lobby code

Ducky will generate a random lobby code during startup. This code will be among the last lines
printed during initialization.

```
[ducky] Lobby code: ABC123
```

## Configuration

Ducky can be configured through a `config.toml` file in the same directory as the executable. If
this file is not found, or if there is a problem during parsing, Ducky will use its [default
configuration](https://github.com/tma02/ducky/blob/master/src/config.rs#L12).

### Example configuration

```toml
name = "A Ducky Server"
motd = "This lobby is powered by Ducky.\nType !help to see commands."
game_version = "1.1"
lobby_code = "HELLOO"
max_players = 25
code_only = true
adult_only = false
ban_list = []
```
