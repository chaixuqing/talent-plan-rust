use failure::Fail;
use serde_json;
use std::{io,string};

#[derive(Fail, Debug)]
#[fail(display = "Error for KvStore")]
pub enum KvError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "{}", _0)]
    Bincode(#[cause] bincode::Error),

    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),

    /// Key or value is invalid UTF-8 sequence
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] string::FromUtf8Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "the command is unknown")]
    UnKnownCommandType,

    #[fail(display = "the engine type is unknown")]
    UnKnownEngineType,

    #[fail(display = "RemoteNetworkError")]
    RemoteNetworkError(String),

    #[fail(display = "RemoteStoreError")]
    RemoteStoreError(String),
}

impl From<io::Error> for KvError {
    fn from(error: io::Error) -> Self {
        KvError::Io(error)
    }
}

impl From<serde_json::Error> for KvError {
    fn from(error: serde_json::Error) -> Self {
        KvError::Serde(error)
    }
}

impl From<bincode::Error> for KvError {
    fn from(e: bincode::Error) -> Self {
        KvError::Bincode(e)
    }
}

impl From<sled::Error> for KvError {
    fn from(e: sled::Error) -> Self {
        KvError::Sled(e)
    }
}

impl From<string::FromUtf8Error> for KvError {
    fn from(e: string::FromUtf8Error) -> Self {
        KvError::Utf8(e)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;
