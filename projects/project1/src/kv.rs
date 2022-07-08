#![deny(missing_docs)]
//! A simple key/value store.

use super::error::Result;
use crate::{KvError};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap},
    fs,
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
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
pub struct KvStore {
    write_buffer: io::BufWriter<fs::File>,
    read_buffer: io::BufReader<fs::File>,
    log_pointer_index: HashMap<String, LogPointer>,
}

const WAL_FILENAME: &str = "wal.log";

#[derive(Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Serialize, Deserialize)]
struct LogPointer {
    offset: u64,
    len: u64,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path.join(WAL_FILENAME))?;
        let write_buffer = io::BufWriter::new(file);
        let read_buffer = io::BufReader::new(fs::File::open(path.join(WAL_FILENAME))?);

        Ok(KvStore {
            write_buffer,
            read_buffer,
            log_pointer_index: HashMap::new(),
        })
    }

    fn load_data(&mut self) -> Result<()> {
        let mut prev_offset = self.read_buffer.seek(SeekFrom::Start(0))?;
        let mut iterator =
            serde_json::Deserializer::from_reader(&mut self.read_buffer).into_iter::<Command>();
        while let Some(command) = iterator.next() {
            let offset = iterator.byte_offset() as u64;
            match command? {
                Command::Set { key, .. } => {
                    self.log_pointer_index.insert(
                        key,
                        LogPointer {
                            offset: prev_offset,
                            len: offset - prev_offset,
                        },
                    );
                }
                Command::Remove { key } => {
                    self.log_pointer_index.remove(&key);
                }
            }
            prev_offset = offset;
        }
        Ok(())
    }
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.load_data()?;
        match self.log_pointer_index.get(&key) {
            Some(log_pointer) => {
                self.read_buffer.seek(SeekFrom::Start(log_pointer.offset))?;
                let command_reader = (&mut self.read_buffer).take(log_pointer.len);
                if let Command::Set { value, .. } = serde_json::from_reader(command_reader)? {
                    Ok(Some(value))
                } else {
                    Err(KvError::UnKnownCommandType)
                }
            }
            None => Ok(None),
        }
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set { key, value };
        serde_json::to_writer(&mut self.write_buffer, &cmd)?;
        self.write_buffer.flush()?;
        Ok(())
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.load_data()?;
        if self.log_pointer_index.contains_key(&key) {
            let cmd = Command::Remove { key };
            serde_json::to_writer(&mut self.write_buffer, &cmd).unwrap();
            self.write_buffer.flush()?;
            Ok(())
        } else {
            Err(KvError::KeyNotFound)
        }
    }
}
