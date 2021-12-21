use log::info;
use std::env;

mod check_and_build;
mod stats;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    info!("Parsing errors with `cargo check`");
    let compiler_errors = check_and_build::parsing_errors(&args[1])?;
    let stats = stats::update_stats(&compiler_errors).context("Stats could not be compiled")?;
    check_and_build::build(&args[1])?;
    stats::print_errors(&stats);
    Ok(())
}
