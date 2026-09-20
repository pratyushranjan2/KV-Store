use std::thread;
use std::time::Duration;
use crate::db::DB;

mod db;
mod file_ops;
mod compaction;

fn main() {
    let mut db = db::DB::new();
    for i in 0..100 {
        db.set(i.to_string(), format!("val_{}", i));
        thread::sleep(Duration::from_secs(1));
    }
}
