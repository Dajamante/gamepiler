use cargo_metadata::Message;
use log::info;
use std::process::{Command, Stdio};
fn main() {
    env_logger::init();

    info!("Trying to read from another rust program");
    let mut command = Command::new("cargo")
        .args(&[
            "check",
            "--message-format=json",
            "--manifest-path=/Users/aissata/Rust/RUSTBOOK/guessing_game/Cargo.toml",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let mut supererror = String::new();
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerMessage(msg) => {
                if let Some(c) = msg.message.code {
                    supererror.push_str(&c.code);
                }
            }
            _ => {} //println!("something"), // Unknown message
        }
    }
    println!("Error code 🍍: {}", supererror);
    let output1 = command.wait().expect("Couldn't get cargo's exit status");

    command = Command::new("cargo")
        .args(&[
            "build",
            "--manifest-path=/Users/aissata/Rust/RUSTBOOK/guessing_game/Cargo.toml",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let output2 = command.wait().expect("Couldn't get cargo's exit status");
}
