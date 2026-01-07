use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct Repository {
    // Define repository fields here
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }
}