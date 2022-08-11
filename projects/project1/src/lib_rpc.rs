use crate::error::{KvError, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(Option<String>),
    NetworkError(String),
    StorageError(String),
}

impl From<Response> for Result<Option<String>> {
    fn from(res: Response) -> Self {
        match res {
            Response::Ok(msg) => Ok(msg),
            Response::NetworkError(e) => Err(KvError::RemoteNetworkError(e)),
            Response::StorageError(e) => Err(KvError::RemoteStoreError(e)),
        }
    }
}

impl From<Response> for Result<()> {
    fn from(res: Response) -> Self {
        match res {
            Response::Ok(_msg) => Ok(()),
            Response::NetworkError(e) => Err(KvError::RemoteNetworkError(e)),
            Response::StorageError(e) => Err(KvError::RemoteStoreError(e)),
        }
    }
}