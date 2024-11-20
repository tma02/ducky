use std::ops::Range;

use rand::{distributions::uniform::{SampleRange, SampleUniform}, prelude::*};

const GODOT_RANDI_RANGE: Range<i64> = 0..i32::MAX as i64;

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
