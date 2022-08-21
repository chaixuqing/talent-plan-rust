use std::path::Path;

use crate::{KvError, Result};
use sled;

use super::kvs_engine::KvsEngine;

#[derive(Clone)]
pub struct SledKvsEngine {
    tree: sled::Db,
}

impl KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.tree.insert(key, value.into_bytes())?;
        self.tree.flush()?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self
            .tree
            .get(key)?
            .map(|v| v.to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&self, key: String) -> Result<()> {
        self.tree.remove(key)?.ok_or(KvError::KeyNotFound)?;
        self.tree.flush()?;
        Ok(())
    }
}

impl SledKvsEngine {
    pub fn new(path: &Path) -> SledKvsEngine {
        SledKvsEngine {
            tree: sled::open(path).unwrap(),
        }
    }
}
