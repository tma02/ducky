use std::time::{SystemTime, UNIX_EPOCH};

pub fn system_time_since_unix_epoch_seconds() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}

pub fn system_time_since_unix_epoch_seconds_float() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        .to_string()
}
