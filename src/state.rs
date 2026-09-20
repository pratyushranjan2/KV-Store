use std::sync::{LazyLock, RwLock};

pub static LATEST_DELETE_TIMESTAMP_FILE: LazyLock<RwLock<String>> = LazyLock::new(|| {
    RwLock::new(String::from("00000000000000000000000.json"))
});