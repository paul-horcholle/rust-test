use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct Repository<T: Clone> {
    // Define repository fields here
    store: Arc<Mutex<HashMap<String, T>>>,
}

impl<T: Clone> Repository<T> {
    pub fn new() -> Self {
        Repository {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn set(&self, key: String, value: T) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }
}