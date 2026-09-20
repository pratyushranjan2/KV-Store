use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio;

mod db;
mod file_ops;
mod compaction;
mod cleanup;
mod state;

#[tokio::main]
async fn main() {
    let mut db = db::DB::new();
    for i in 0..100 {
        db.set(i.to_string(), format!("val_{}", i));
        //thread::sleep(Duration::from_secs(1));
    }

    let db = Arc::new(db);

    for i in 0..100 {
        let db_clone = Arc::clone(&db);
        tokio::spawn(async move {
            for j in 0..10 {
                let val = db_clone.get(i.to_string()).await.unwrap();
                println!("{} -> {}", i, val);
                tokio::time::sleep(Duration::from_secs(6)).await;
            }
        });
    }
    thread::sleep(Duration::from_secs(65));
}
