use chrono::{DateTime, Local};
use std::process::Command;
use std::fs;

fn main() {
    let metadata = fs::metadata("/").expect("Failed to get root directory metadata");
    let created = metadata.created().expect("Failed to get creation time");
    let install_time = DateTime::<Local>::from(created);
    let current_time = Local::now();
    let duration = current_time.timestamp() - install_time.timestamp();
    
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 3600) % 24;
    let days = (duration / 86400) % 30;
    let months = (duration / 2592000) % 12;
    let years = duration / 31536000;
    
    let mut output = String::new();
    if years > 0 {
        output.push_str(&format!("{}y ", years));
    }
    if months > 0 || years > 0 {
        output.push_str(&format!("{}m ", months));
    }
    if days > 0 || months > 0 || years > 0 {
        output.push_str(&format!("{}d ", days));
    }
    if hours > 0 || days > 0 || months > 0 || years > 0 {
        output.push_str(&format!("{}h ", hours));
    }
    if minutes > 0 || hours > 0 || days > 0 || months > 0 || years > 0 {
        output.push_str(&format!("{}m ", minutes));
    }
    output.push_str(&format!("{}s", seconds));

    let username = Command::new("whoami")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| String::from("unknown"))
        .trim()
        .to_string();

    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| String::from("unknown"))
        .trim()
        .to_string();

    println!("Archlinux@{} {} has been installed for {}.", username, kernel, output);
}
