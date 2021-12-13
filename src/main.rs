use log::info;
use serde_json::{Result as SerdeResult, Value};
use std::io::{self, BufRead};

fn main() {
    env_logger::init();
    info!("Trying to read from another rust program");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Reading from another program failed");
        //println!("GAMEPILER 🍍 {:#?}", line);
        if let Ok(v) = find_code_error(&line) {
            println!("GAMEPILER 🍍: {}", v);
        };
    }
}

fn find_code_error(line: &str) -> SerdeResult<Value> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(line)?;

    Ok(v["message"]["rendered"].clone())
}
