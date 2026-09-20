use crate::file_ops;

use crate::state::LATEST_DELETE_TIMESTAMP_FILE;
use chrono::{Duration, Local};
use core::time::Duration as StdDuration;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::thread;

// file_path1 is older
fn compact(file_path1: &PathBuf, file_path2: &PathBuf) {
    println!("Starting compaction among {} and {}", file_path1.display(), file_path2.display());
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

    let delete_file_name = file_path1.file_name().unwrap().to_str().unwrap().to_string();
    // update LATEST_DELETE_TIMESTAMP to fi
    let mut guard = LATEST_DELETE_TIMESTAMP_FILE.write().unwrap();
    *guard = delete_file_name.clone();
    drop(guard);

    let mut delete_info_map = file_ops::read_delete_info_into_map();
    let deletion_timestamp = Local::now() + Duration::seconds(10);
    let deletion_timestamp_str = deletion_timestamp.format("%Y%m%d%H%M%S%f").to_string();
    delete_info_map.insert(delete_file_name, deletion_timestamp_str);
    file_ops::write_delete_info_into_file(delete_info_map);
}

fn try_perform_compaction() {
    let file_paths: Vec<PathBuf> = file_ops::get_sorted_json_files("data").unwrap();

    // filter file_paths to get only values greater than LATEST_DELETE_TIMESTAMP
    let latest_delete_timestamp = LATEST_DELETE_TIMESTAMP_FILE.read().unwrap();
    println!("latest delete timestamp: {}", latest_delete_timestamp);
    let filtered_paths: Vec<_> = file_paths.iter().filter(|p| {
        p.file_name().and_then(|n| n.to_str()).map(|s| s > latest_delete_timestamp.as_str())
            .unwrap_or(false)
    }).collect();
    drop(latest_delete_timestamp);

    println!("Filtered paths len: {}", filtered_paths.len());
    if filtered_paths.len() >= 2 {
        let file_path1 = filtered_paths.get(filtered_paths.len() - 1).unwrap();
        let file_path2 = filtered_paths.get(filtered_paths.len() - 2).unwrap();

        compact(file_path1, file_path2);
    }
}

pub fn start_compaction_worker() {
    initialize_latest_delete_timestamp();
    thread::spawn(|| {
        loop {
            thread::sleep(StdDuration::from_secs(5));
            try_perform_compaction();
        }
    });
}

fn initialize_latest_delete_timestamp() {
    let delete_map = file_ops::read_delete_info_into_map();
    if !delete_map.is_empty() {
        let max_timestamp = delete_map.values().max().unwrap();
        let mut guard = LATEST_DELETE_TIMESTAMP_FILE.write().unwrap();
        *guard = max_timestamp.to_string();
        drop(guard)
    }
}