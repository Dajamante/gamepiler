use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

pub fn read_data() -> HashMap<String, i32> {
    if let Ok(error_map) = File::open(Path::new("error_map.json")) {
        serde_json::from_reader(&error_map).unwrap()
    } else {
        Default::default()
    }
}

pub fn write_data(map: &HashMap<String, i32>) {
    let error_map = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("error_map.json")
        .unwrap();

    let _serialized = serde_json::to_writer(&error_map, &map);
}
