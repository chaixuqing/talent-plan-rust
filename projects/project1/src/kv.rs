#![deny(missing_docs)]
//! A simple key/value store.

use std::{collections::HashMap, path::PathBuf};
use super::error::{Result,KvError};
/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    // pub fn new() -> KvStore {
        
    // }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>{
        Ok(KvStore {
            store: HashMap::new(),
        })
    }
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // self.store.get(&key).cloned()
        Err(KvError::Unimplemented)
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // self.store.insert(key, value);
        Err(KvError::Unimplemented)
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()>
    {
        // self.store.remove(&key);
        Err(KvError::Unimplemented)
    }
}
