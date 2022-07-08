use failure::Fail;
use serde_json;
use std::io;

#[derive(Fail,Debug)]
#[fail(display="Error for KvStore")]
pub enum KvError{
    #[fail(display="{}",_0)]
    Io(#[cause] io::Error),
    
    #[fail(display="{}",_0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display="Key not found")]
    KeyNotFound,

    #[fail(display="the command is unknown")]
    UnKnownCommandType,
}

impl From<io::Error> for KvError{
    fn from(error: io::Error) -> Self {
        KvError::Io(error)
    }
}

impl From<serde_json::Error> for KvError{
    fn from(error: serde_json::Error) -> Self {
        KvError::Serde(error)
    }
}

pub type Result<T> = std::result::Result<T,KvError>;