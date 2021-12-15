use cargo_metadata::Message;
use inline_python::python;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
fn read_data() -> HashMap<String, i32> {
    if let Ok(error_map) = File::open(Path::new("error_map.json")) {
        //serde_json::from_reader(&error_map).unwrap()
        serde_json::from_reader::<_, HashMap<String, i32>>(&error_map).unwrap()
        //serde_json::from_reader::<HashMap<String, i32>>(&error_map).unwrap()
    } else {
        Default::default()
    }
}

fn write_data(map: &HashMap<String, i32>) {
    let error_map = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("error_map.json")
        .unwrap();

    let serialized = serde_json::to_writer(&error_map, &map);
    //println!("Serialized: {:#?}", serialized.unwrap());
}

fn parsing_errors(man_path: &str) -> Vec<String> {
    info!("Trying to read from another rust program");
    let mut command = Command::new("cargo")
        .args(&[
            "check",
            "--message-format=json",
            //"--manifest-path=/Users/aissata/Rust/RUSTBOOK/guessing_game/Cargo.toml",
            man_path,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let mut compiler_errors: Vec<String> = Vec::new();
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerMessage(msg) => {
                println!("{}", &msg);
                if let Some(c) = msg.message.code {
                    //println!("{}", &c.code);
                    if c.code.len() == 5 && c.code.starts_with("E") {
                        compiler_errors.push(c.code);
                    }
                }
            }
            _ => {} //println!("something"), // Unknown message
        }
    }
    // compiler_errors
    //     .into_iter()
    //     .for_each(|e| println!("GAMEPILER found error with code 🍍: {}", e));

    command.wait().expect("Couldn't get cargo's exit status");
    compiler_errors
}
/// To do: fix path manifest to make to take any path
/// Make a persistent file to store errors
fn main() {
    env_logger::init();

    info!("Getting the json file to reccord errors.");
    //println!("{:#?}", error_map);
    let mut man_path = "--manifest-path=".to_string();
    man_path.push_str("/Users/aissata/Rust/RUSTBOOK/guessing_game/Cargo.toml");
    let compiler_errors = parsing_errors(&man_path);
    let mut error_map: HashMap<String, i32> = read_data();
    compiler_errors.iter().for_each(|e| {
        let stat = error_map.entry(e.to_string()).or_insert(0);
        *stat += 1;
    });
    write_data(&error_map);
    // save into file
    error_map
        .iter()
        .for_each(|(k, v)| println!("GAMEPILER: 🍍 You commited error {} {} times, yay!", k, v));
    println!("All errors file: {:#?}", error_map);

    python! {
        import matplotlib.pyplot as plt
        import numpy as np

        with plt.xkcd():
            fig = plt.figure()
            ax = fig.add_axes((0.1, 0.2, 0.8, 0.7))
            labels = []
            sizes = []

            for x, y in 'error_map.items():
                labels.append(x)
                sizes.append(y)

            # Plot
            ax.pie(sizes, labels=labels)
            fig.text(
                0.5, 0.05,
                "THE DAY I REALISED I COULD \n PLOT MY RUSTC ERRORS \\(^ ^)/", ha="center")
        plt.show()
    }
    Command::new("cargo")
        .args(&["build", &man_path])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
        .wait()
        .expect("Couldn't get cargo's exit status");
}
