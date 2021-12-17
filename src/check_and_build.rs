use cargo_metadata::Message;
use log::info;
use std::process::{Command, Stdio};

fn get_manifest_arg(path_to_manifest: &str) -> String {
    "--manifest-path=".to_string() + path_to_manifest
}
pub fn parsing_errors(path_to_manifest: &str) -> Vec<String> {
    info!("Trying to read from another rust program");
    let mut command = Command::new("cargo")
        .args(&[
            "check",
            "--message-format=json",
            &get_manifest_arg(path_to_manifest),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let mut compiler_errors: Vec<String> = Vec::new();
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerMessage(msg) => {
                //println!("{}", &msg);
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
    compiler_errors
        .iter()
        .for_each(|e| println!("GAMEPILER found error with code 🍍: {}", e));

    command.wait().expect("Couldn't get cargo's exit status");
    compiler_errors
}

pub fn build(path_to_manifest: &str) {
    info!("Building the file with `cargo build`");

    Command::new("cargo")
        .args(&["build", &get_manifest_arg(path_to_manifest)])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
        .wait()
        .expect("Couldn't get cargo's exit status");
}
