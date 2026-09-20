use crate::file_ops;
use crate::compaction;
use crate::cleanup;

use std::collections::HashMap;

const DUMP_THRESHOLD: u32 = 3;

pub struct DB {
    store: HashMap<String, String>,
}

impl DB {
    pub fn new() -> DB {
        compaction::start_compaction_worker();
        cleanup::start_cleanup_worker();
        DB { store: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
        
        if self.store.len() as u32 >= DUMP_THRESHOLD {
            file_ops::dump_store_new_file(&mut self.store);
        }
    }

    pub async fn get(&self, key: String) -> Option<String> {
        if let Some(value) = self.store.get(&key) {
            return Some(value.clone());
        }
        
        if let Some(value) = file_ops::find_key_in_dump(&key) {
            return Some(value)
        }
        None
    }
}