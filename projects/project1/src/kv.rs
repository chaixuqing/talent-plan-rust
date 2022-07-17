#![deny(missing_docs)]
//! A simple key/value store.

use super::error::Result;
use crate::KvError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
pub struct KvStore {
    write_buffer: io::BufWriter<fs::File>,
    read_buffer: io::BufReader<fs::File>,
    log_pointer_index: HashMap<String, LogPointer>,
    log_file_size: u64,
    log_file_id: u64,
    path: PathBuf,
}

const WAL_FILENAME_SUFFIX: &str = "log";
const COMPACT_THRESHOLD: u64 = 1024 * 256; // 256KB

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
        let log_file_id = get_logfile(&path)?;
        let log_file = log_path(&path, log_file_id);
        let file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_file.clone())?;
        let write_buffer = io::BufWriter::new(file);
        let read_buffer = io::BufReader::new(fs::File::open(log_file)?);

        let mut kv_store = KvStore {
            write_buffer,
            read_buffer,
            log_pointer_index: HashMap::new(),
            log_file_size: 0,
            log_file_id,
            path,
        };
        kv_store.load_data()?;
        Ok(kv_store)
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
        self.log_file_size = prev_offset;
        if self.log_file_size > COMPACT_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
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
        let cmd = Command::Set {
            key: key.clone(),
            value,
        };
        let offset = self.write_buffer.seek(SeekFrom::End(0))?;
        serde_json::to_writer(&mut self.write_buffer, &cmd)?;
        self.write_buffer.flush()?;
        let len = self.write_buffer.seek(SeekFrom::End(0))? - offset;
        self.log_file_size += len;
        self.log_pointer_index
            .insert(key, LogPointer { offset, len });
        if self.log_file_size > COMPACT_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.log_pointer_index.contains_key(&key) {
            let cmd = Command::Remove { key: key.clone() };
            let offset = self.write_buffer.seek(SeekFrom::End(0))?;
            serde_json::to_writer(&mut self.write_buffer, &cmd).unwrap();
            self.write_buffer.flush()?;
            let len = self.write_buffer.seek(SeekFrom::End(0))? - offset;
            self.log_file_size += len;
            self.log_pointer_index.remove(&key);
            if self.log_file_size > COMPACT_THRESHOLD {
                self.compact()?;
            }
            Ok(())
        } else {
            Err(KvError::KeyNotFound)
        }
    }

    fn compact(&mut self) -> Result<()> {
        let new_log_file_id = self.log_file_id + 1;
        let compact_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_path(&self.path, new_log_file_id))?;
        let mut compact_writer = io::BufWriter::new(compact_file);
        let mut prev_offset = compact_writer.seek(SeekFrom::Current(0))?;
        for log_pointer in self.log_pointer_index.values_mut() {
            self.read_buffer.seek(SeekFrom::Start(log_pointer.offset))?;
            let mut cmd_reader = (&mut self.read_buffer).take(log_pointer.len);
            io::copy(&mut cmd_reader, &mut compact_writer)?;
            let cur_offset = compact_writer.seek(SeekFrom::Current(0))?;
            log_pointer.offset = prev_offset;
            prev_offset = cur_offset;
        }
        compact_writer.flush()?;
        self.log_file_size = prev_offset;
        fs::remove_file(log_path(&self.path, self.log_file_id))?;
        self.log_file_id = new_log_file_id;
        self.read_buffer =
            io::BufReader::new(fs::File::open(log_path(&self.path, self.log_file_id))?);
        self.write_buffer = compact_writer;
        Ok(())
    }
}

/// Search files with pattern "<file_id>.log" and return the file with the lowest log id
///
/// # Errors
/// It propagates I/O errors.
fn get_logfile(path: impl Into<PathBuf>) -> Result<u64> {
    let path = path.into();
    let mut file_ids: Vec<u64> = fs::read_dir(path.clone())?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some(OsStr::new(WAL_FILENAME_SUFFIX)))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(&format!(".{}", WAL_FILENAME_SUFFIX)))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    file_ids.sort();
    if file_ids.len() == 0 {
        // can't found any log file
        fs::File::create(log_path(&path, 0))?;
        return Ok(0);
    }
    for index in 1..file_ids.len() {
        fs::remove_file(log_path(&path, file_ids[index]))?;
    }
    Ok(file_ids[0])
}

fn log_path(path: &PathBuf, file_id: u64) -> PathBuf {
    path.join(format!("{}.{}", file_id, WAL_FILENAME_SUFFIX))
}
