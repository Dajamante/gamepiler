use anyhow::{ensure, Context, Result};
use cargo_metadata::Message;
use log::info;
use std::process::{Command, Stdio};

fn get_manifest_arg(path_to_manifest: &str) -> Result<String> {
    ensure!(!path_to_manifest.is_empty());
    Ok("--manifest-path=".to_string() + path_to_manifest)
}
pub fn parsing_errors(path_to_manifest: &str) -> Result<Vec<String>> {
    info!("Trying to read from another rust program");
    let mut command = Command::new("cargo")
        .args(&[
            "check",
            "--message-format=json",
            &get_manifest_arg(path_to_manifest).context("No path was found, check failed.")?,
        ])
        .stdout(Stdio::piped())
        .spawn()
        //.context("Failed")?;
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let mut compiler_errors: Vec<String> = Vec::new();
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerMessage(msg) => {
                if let Some(c) = msg.message.code {
                    if c.code.len() == 5 && c.code.starts_with("E") {
                        compiler_errors.push(c.code);
                    }
                }
            }
            _ => {}
        }
    }
    compiler_errors
        .iter()
        .for_each(|e| println!("GAMEPILER found error with code 🍍: {}", e));

    command.wait().expect("Couldn't get cargo's exit status");
    Ok(compiler_errors)
}

pub fn build(path_to_manifest: &str) -> Result<()> {
    info!("Building the file with `cargo build`");

    Command::new("cargo")
        .args(&[
            "build",
            &get_manifest_arg(path_to_manifest).context("No path was found, build failed")?,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e))
        .wait()
        .expect("Couldn't get cargo's exit status");

    Ok(())
}
