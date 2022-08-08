use crate::KvsEngine;
use super::error::Result;

pub struct SledKvsEngine {}

impl KvsEngine for SledKvsEngine{
    fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
}