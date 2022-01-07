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
    include_str,
};
const ERROR_CATEGORIES: &str = include_str!("compiler_error_categories.json");

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
    let h = Histogram::from_slice(&new_stuff[..], HistogramBins::Count(25));
    let v = ContinuousView::new()
        .add(h)
        .x_label("Something")
        .y_label("Other thing");

    println!("{}", Page::single(&v).dimensions(50, 15).to_text().unwrap());
}

pub fn get_categories() -> HashMap<String, String> {
    serde_json::from_str(ERROR_CATEGORIES).unwrap()
}

// this is needed while researching how to classify categories!
pub fn compiler_errors_categories() -> HashSet<String> {
    let cat_map: HashMap<String, String> = get_categories();
    let unique_categories = cat_map
        .into_iter()
        .map(|(_, v)| v)
        //.unique_by(|(k, v)| v)
        .collect::<HashSet<String>>();
    unique_categories
}
pub fn graph_xkcd(stats: &Stats) {
    let error_map = stats.error_map.clone();
    let categories = get_categories();
    let mut freq: HashMap<String, i32> = HashMap::new();
    for (k, _) in &error_map {
        if let Some(k2) = categories.get(k) {
            *freq.entry(k2.to_string()).or_default() += 1;
        }
    }
    python! {
        import matplotlib.pyplot as plt
        import numpy as np
        from matplotlib import colors
        from matplotlib.pyplot import figure


        with plt.xkcd():
            fig, (ax1, ax2) = plt.subplots(nrows=1,ncols=2, figsize=(12,8))
            fig.suptitle("THE DAY I REALISED I COULD \n PLOT MY RUSTC ERRORS \\(^ ^)/", ha="center")
            labels = []
            sizes = []

            for x, y in 'freq.items():
                labels.append(x)
                sizes.append(y)

            # Plot
            my_cmap = plt.get_cmap("viridis")
            rescale = lambda y: (y - np.min(y)) / (np.max(y) - np.min(y))

            ax1.bar('error_map.keys(), 'error_map.values(), color = my_cmap(rescale(list('error_map.values()))))
            ax1.set_title("The said errors...")
            ax2.pie(sizes, labels=labels)
            ax2.set_title("... and what they \"mean\"\n. Sort of.")

            plt.show()
    }
}
