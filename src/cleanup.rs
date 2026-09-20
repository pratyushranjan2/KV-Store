use std::{fs, thread};
use std::fs::File;
use std::time::Duration;
use chrono::Local;
use crate::file_ops;

fn try_perform_cleanup() {
    let mut deletion_map = file_ops::read_delete_info_into_map();
    let current_timestamp = Local::now().format("%Y%m%d%H%M%S%f").to_string();

    deletion_map.retain(|file_name, delete_at| {
        if delete_at.as_str() <= current_timestamp.as_str() {
            let file_path = format!("data/{}", file_name);
            if let Ok(file) = File::open(&file_path) {
                drop(file);
                fs::remove_file(&file_path).unwrap();
                println!("File {} removed", &file_path);
                return false;
            }
        }
        true
    });

    file_ops::write_delete_info_into_file(deletion_map);
}

pub fn start_cleanup_worker() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(6));
            try_perform_cleanup();
        }
    });
}