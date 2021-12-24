use clap::{App, Arg};
use log::info;

mod check_and_build;
mod stats;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    env_logger::init();

    let matches = App::new("Gamepiler 🍍")
        .version("1.0")
        .about("Gives achievement for your errors!")
        .author("Ai Maiga")
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .help("Plots a graph.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("xkcd")
                .short("x")
                .long("xkcd")
                .help("Plots a graph in xkcd style")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    info!("Parsing errors with `cargo check`");
    let compiler_errors = check_and_build::parsing_errors()?;

    let stats = stats::update_stats(&compiler_errors).context("Stats could not be compiled")?;
    check_and_build::build()?;

    stats::print_errors(&stats);
    if matches.is_present("graph") {
        stats::graph(&stats);
    };
    if matches.is_present("xkcd") {
        stats::graph_xkcd(&stats);
    };
    Ok(())
}
