use std::path::Path;

use super::error::Result;
use crate::{KvError, KvsEngine};
use sled;

pub struct SledKvsEngine {
    tree: sled::Db,
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.tree.insert(key, value.into_bytes())?;
        self.tree.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self
            .tree
            .get(key)?
            .map(|v| v.to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
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
