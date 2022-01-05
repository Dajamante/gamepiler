use anyhow::{Context, Result};
use directories::ProjectDirs;
use inline_python::python;
use log::{info, warn};
use plotlib::{
    page::Page,
    repr::{Histogram, HistogramBins},
    view::ContinuousView,
};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
};

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
    fn get_file_path_errors() -> Result<std::path::PathBuf> {
        let proj_dirs =
            ProjectDirs::from("", "", "gamepile").context("Did not find a home directory.")?;

        if !proj_dirs.data_local_dir().exists() {
            fs::create_dir(proj_dirs.data_local_dir())
                .context("Could not create gamepile directory!")?;
        }
        Ok(proj_dirs.data_local_dir().join("error_file.json"))
        // "/Users/aissata/Rust/gamepiler/error_file.json".to_string()
    }
}

fn load_from_file() -> Result<Stats> {
    if let Ok(error_file) = File::open(Stats::get_file_path_errors()?.as_path()) {
        let error_map = serde_json::from_reader(&error_file).context("Could not serialise.")?;
        Ok(Stats { error_map })
    } else {
        warn!("The file error does not exist, are you sure of yourself?");
        Ok(Stats::new())
    }
}

fn save_to_file(stats: &Stats) -> Result<()> {
    let error_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(Stats::get_file_path_errors()?.as_path())
        .context("Could not open error file shit")?;

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
pub fn graph(stats: &Stats) {
    let mut new_stuff: Vec<f64> = vec![];
    for (_k, v) in &stats.error_map {
        new_stuff.push(*v as f64);
    }
    // let bins = new_stuff.len();
    // println!("New stuff: {:#?}", new_stuff);
    // println!("Stats error map: {:#?}", stats.error_map);
    let h = Histogram::from_slice(&new_stuff[..], HistogramBins::Count(25));
    let v = ContinuousView::new()
        .add(h)
        .x_label("Something")
        .y_label("Other thing");

    println!("{}", Page::single(&v).dimensions(50, 15).to_text().unwrap());
}
pub fn compiler_errors() {
    let path = "/Users/aissata/Rust/gamepiler/compiler_error_categories.json".to_string();
    if let Ok(error_file) = File::open(path) {
        let error_map = serde_json::from_reader::<_, serde_json::Value>(&error_file)
            .into_iter()
            .map(|v| v.to_string())
            .collect::<HashSet<String>>();
        println!("{:#?}", error_map);
    }
}
pub fn graph_xkcd(stats: &Stats) {
    let error_map = stats.error_map.clone();
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
}
