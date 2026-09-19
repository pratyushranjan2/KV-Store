use std::collections::HashMap;
use std::{fs, io};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use chrono::Local;

pub fn dump_store_new_file(store: &mut HashMap<String, String>) {
    let timestamp = Local::now().format("%Y%m%d%H%M%S%f").to_string();
    let filename = format!("data/{}.json", timestamp);

    fs::create_dir_all("data").unwrap();
    let file = File::create(filename).unwrap();
    serde_json::to_writer_pretty(file, &store).unwrap();

    store.clear();
}

fn get_sorted_json_files(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir_path)?;

    let mut files: Vec<PathBuf> = entries
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "json"))
        .collect();

    // Sort descending by filename (most recent first)
    files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    Ok(files)
}

pub fn find_key_in_dump(key: &String) -> Option<String> {
    let file_paths = get_sorted_json_files("data").unwrap();

    for path in file_paths {
        println!("Searching for key {} in file {}", key, path.file_name().unwrap().to_str().unwrap());
        
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let map = serde_json::from_reader::<_, HashMap<String, String>>(reader)
            .unwrap();

        if let Some(value) = map.get(key) {
            return Some(value.clone());
        }
    }
    
    None
}