use log::info;
use std::collections::HashMap;
use std::env;

mod check_build;
mod stats;

// error management
// command line arguments
// better structure management? reduce possible errors?
// modules

/// To do: fix path manifest to make to take any path
/// Make a persistent file to store errors
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    info!("Getting the json file to reccord errors.");
    let compiler_errors = check_build::parsing_errors(&args[1]);
    let mut error_map: HashMap<String, i32> = stats::read_data();
    compiler_errors.iter().for_each(|e| {
        let stat = error_map.entry(e.to_string()).or_insert(0);
        *stat += 1;
    });
    stats::write_data(&error_map);
    // save into file
    //check_build::build(&args[1]);
    print_errors(error_map);
}

fn print_errors(error_map: HashMap<String, i32>) {
    error_map
        .iter()
        .for_each(|(k, v)| println!("GAMEPILER: 🍍 You commited error {} {} times, yay!", k, v));
    //println!("All errors file: {:#?}", error_map);
}
