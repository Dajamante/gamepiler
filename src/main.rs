use log::info;
use std::process::Command;

fn main() {
    env_logger::init();
    info!("Trying to read from another rust program");
    let output = Command::new("rustc")
        .arg("/Users/aissata/Rust/RUSTBOOK/guessing_game/src/main.rs")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("GAMEPILER says 🍍: \n{}", s);
    }
}
