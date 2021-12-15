use cargo_metadata::Message;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn read_data() -> HashMap<String, i32> {
    if let Ok(error_map) = File::open(Path::new("error_map.json")) {
        println!("{:#?}", Path::new("error_map.json"));
        println!("error map: {:#?}", error_map);
        serde_json::from_reader(&error_map).unwrap()
    } else {
        Default::default()
    }
}

fn write_data(map: &HashMap<String, i32>) {
    //let serialized = serde_json::to_string(&map).unwrap();
    //serde_json::to_writer(error_map, serialized.as_bytes())
    let mut error_map = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("error_map.json")
        .unwrap();

    //error_map.write_all(serialized.as_bytes()).unwrap();
    let serialized = serde_json::to_writer(&error_map, &map);
    println!("Serialized: {:#?}", serialized.unwrap());
}

/// To do: fix path manifest to make to take any path
/// Make a persistent file to store errors
fn main() {
    env_logger::init();

    info!("Getting the json file to reccord errors.");
    //println!("{:#?}", error_map);

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
    println!("GAMEPILER found error with code 🍍: {}", supererror);

    command.wait().expect("Couldn't get cargo's exit status");

    let mut error_map: HashMap<String, i32> = read_data();
    let stat = error_map.entry(supererror).or_insert(0);
    *stat += 1;
    write_data(&error_map);
    // save into file
    println!("Greeting file: {:#?}", &error_map);

    command = Command::new("cargo")
        .args(&[
            "build",
            "--manifest-path=/Users/aissata/Rust/RUSTBOOK/guessing_game/Cargo.toml",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    command.wait().expect("Couldn't get cargo's exit status");
}
