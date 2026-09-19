use crate::db::DB;

mod db;
mod file_ops;

fn main() {
    let mut db = db::DB::new();
    for i in 0..12 {
        db.set(i.to_string(), format!("val_{}", i));
    }

    println!("{}", db.get("1".to_string()).unwrap());
}
