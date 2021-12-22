use log::info;
use std::env;

mod check_and_build;
mod stats;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    args.iter().for_each(|a| println!("{}", a));
    info!("Parsing errors with `cargo check`");
    //let compiler_errors = check_and_build::parsing_errors(&args[1])?;
    let compiler_errors = check_and_build::parsing_errors()?;

    let stats = stats::update_stats(&compiler_errors).context("Stats could not be compiled")?;
    //check_and_build::build(&args[1])?;
    check_and_build::build()?;

    stats::print_errors(&stats);
    Ok(())
}
