use ansi_term::Color;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> u64 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    timestamp.as_secs() as u64
}

pub fn mark_line(title: &str) {
    println!(
        " ============================ {} =====================================",
        Color::Green.bold().paint(title)
    );
}
