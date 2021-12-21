use anyhow::{Context, Result};
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
fn load_from_file() -> Result<Stats> {
    if let Ok(error_file) = File::open(Path::new("error_file.json")) {
        let error_map = serde_json::from_reader(&error_file).context("Could not serialise.")?;
        Ok(Stats { error_map })
    } else {
        warn!("The file error does not exist, are you sure?");
        Ok(Stats::new())
    }
}

fn save_to_file(stats: &Stats) -> Result<()> {
    let error_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("error_file.json")
        .context("Could not open error file")?;

    serde_json::to_writer(&error_file, &stats.error_map)
        .context("Could not write to error file")?;

    Ok(())
}

pub fn print_errors(stats: &Stats) {
    stats
        .error_map
        .iter()
        .for_each(|(k, v)| println!("GAMEPILER: 🍍 You commited error {} {} times, yay!", k, v));
}

pub fn update_stats(compiler_errors: &Vec<String>) -> Result<Stats> {
    let mut stats: Stats = load_from_file()?;
    stats.update_errors(&compiler_errors);
    info!("Saving errors to permanent file.");
    save_to_file(&stats)?;
    Ok(stats)
}
