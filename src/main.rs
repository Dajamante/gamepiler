use log::info;
use std::io::{self, BufRead};

fn main() {
    env_logger::init();
    info!("Trying to read from another rust program");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Reading from another program failed");
        println!("GAMEPILER 🍍: {}", line);
    }
}
