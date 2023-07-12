use ansi_term::{Color, Style};
use chrono::{DateTime, Local};

pub fn current_timestamp() -> u64 {
    let local: DateTime<Local> = Local::now();
    local.timestamp_millis() as u64
}
pub fn now_string() -> String {
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    local.to_string()
}

pub fn mark_line(title: &str) {
    let style: Style = Color::Green.bold();
    let add_space = format!("  {}  ", &title);
    let padding_str = format!("{:*^72}", add_space);
    println!("\n[{}] - {}", now_string(), style.paint(padding_str));
    // println!("{:10}", "hello");
    // println!("{:*<10}", "hello");
    // println!("{:*>10}", "hello");
    // println!("{:*^30}", "hello world");
}
