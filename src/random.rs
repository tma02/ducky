use std::ops::Range;

use rand::prelude::*;

const GODOT_RANDI_RANGE: Range<i64> = 0..i32::MAX as i64;
static LOBBY_CODE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Mimics the Godot randi() function. Returns a random number between 0 and i32::MAX as an i64.
pub fn godot_randi() -> i64 {
    rand::thread_rng().gen_range(GODOT_RANDI_RANGE)
}

pub fn godot_rand_range(start: f64, end: f64) -> f64 {
    rand::thread_rng().gen_range(start..end)
}

pub fn godot_randf() -> f64 {
    godot_rand_range(0.0, 1.0)
}

/// Returns a six-digit alphanumeric code
pub fn lobby_code() -> String {
    let mut rng = thread_rng();
    
    (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..LOBBY_CODE_CHARSET.len());
            LOBBY_CODE_CHARSET[idx] as char
        })
        .collect()
}

pub fn lobby_server_browser_value() -> String {
    rand::thread_rng().gen_range(0..20).to_string()
}
