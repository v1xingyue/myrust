use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> u64 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    timestamp.as_secs() as u64
}
