use log::{info, warn};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

pub struct Stats {
    error_map: HashMap<String, i32>,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            error_map: HashMap::new(),
        }
    }
    pub fn update_errors(&mut self, compiler_errors: &Vec<String>) {
        info!("Counting errors and updating stats.");
        compiler_errors.iter().for_each(|e| {
            let stat = self.error_map.entry(e.to_string()).or_insert(0);
            *stat += 1;
        });
    }
}
fn load_from_file() -> Stats {
    if let Ok(error_file) = File::open(Path::new("error_file.json")) {
        let error_map = serde_json::from_reader(&error_file).unwrap();
        Stats { error_map }
    } else {
        warn!("The file error does not exist, are you sure?");
        Stats::new()
    }
}

fn save_to_file(stats: &Stats) {
    let error_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("error_file.json")
        .unwrap();

    let serialized = serde_json::to_writer(&error_file, &stats.error_map);
}

pub fn print_errors(stats: &Stats) {
    stats
        .error_map
        .iter()
        .for_each(|(k, v)| println!("GAMEPILER: 🍍 You commited error {} {} times, yay!", k, v));
}

pub fn update_stats(compiler_errors: &Vec<String>) -> Stats {
    let mut stats: Stats = load_from_file();
    stats.update_errors(&compiler_errors);
    info!("Saving errors to permanent file.");
    save_to_file(&stats);
    stats
}
