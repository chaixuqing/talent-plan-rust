use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{KvStore, KvsEngine};

pub mod kv;
pub mod kvs_engine;
pub mod sled_kvs_engine;

#[derive(Clone)]
pub struct ArcKvStore {
    kv_store: Arc<Mutex<KvStore>>,
}

impl ArcKvStore {
    pub fn new(path: impl Into<PathBuf>) -> ArcKvStore {
        ArcKvStore {
            kv_store: Arc::new(Mutex::new(KvStore::open(path).unwrap())),
        }
    }
}

impl KvsEngine for ArcKvStore {
    fn set(&self, key: String, value: String) -> crate::Result<()> {
        self.kv_store.lock().unwrap().set(key, value)
    }

    fn get(&self, key: String) -> crate::Result<Option<String>> {
        self.kv_store.lock().unwrap().get(key)
    }

    fn remove(&self, key: String) -> crate::Result<()> {
        self.kv_store.lock().unwrap().remove(key)
    }
}
