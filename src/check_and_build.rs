use anyhow::{Context, Result};
use cargo_metadata::Message;
use log::info;
use std::process::{Command, Stdio};

pub fn parsing_errors() -> Result<Vec<String>> {
    info!("Trying to read from another rust program");
    let mut command = Command::new("cargo")
        .args(&["check", "--message-format=json"])
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to execute the check!")?;

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

    command.wait().expect("Couldn't get cargo's exit status");
    Ok(compiler_errors)
}

pub fn build() -> Result<()> {
    info!("Building the file with `cargo build`");

    Command::new("cargo")
        .args(&["build"])
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to execute build!")?
        .wait()
        .context("Couldn't get cargo's exit status")?;

    Ok(())
}
