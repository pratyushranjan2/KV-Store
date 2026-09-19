use crate::file_ops;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

// file_path1 is older
fn compact(file_path1: &PathBuf, file_path2: &PathBuf) {
    let overwrite_file_path = file_path2.clone();

    let file1 = File::open(file_path1).unwrap();
    let file2 = File::open(file_path2).unwrap();

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let mut map1: HashMap<String, String> = serde_json::from_reader(reader1).unwrap();
    let map2: HashMap<String, String> = serde_json::from_reader(reader2).unwrap();

    for (k, v) in map2 {
        map1.insert(k, v);
    }

    let file = File::create(overwrite_file_path).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &map1).unwrap();
    writer.flush().unwrap();
}

pub fn try_perform_compaction() {
    let file_paths: Vec<PathBuf> = file_ops::get_sorted_json_files("data").unwrap();
    if file_paths.len() >= 2 {
        let file_path1 = file_paths.get(file_paths.len() - 1).unwrap();
        let file_path2 = file_paths.get(file_paths.len() - 2).unwrap();

        compact(file_path1, file_path2);
    }
}