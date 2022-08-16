use crate::{
    lib_rpc::{Request, Response},
    KvsEngine, Result,
};
use log::{info, warn};
use std::net;

pub struct KvsServer<T: KvsEngine> {
    addr: net::SocketAddr,
    engine: T,
}

impl<T: KvsEngine> KvsServer<T> {
    pub fn new(addr: net::SocketAddr, engine: T) -> KvsServer<T> {
        KvsServer { addr, engine }
    }

    pub fn start(&mut self) -> Result<()> {
        let listener = net::TcpListener::bind(self.addr)?;
        // let mut thread_poll: Vec<thread::JoinHandle<()>> = Vec::new();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!(
                        "get a connection from {}",
                        stream.peer_addr().unwrap().to_string()
                    );
                    let request = bincode::deserialize_from(&mut stream)?;
                    match request {
                        Request::Get { key } => {
                            let result = match self.engine.get(key) {
                                Ok(opt) => Response::Ok(opt),
                                Err(e) => Response::StorageError(e.to_string()),
                            };
                            bincode::serialize_into(&mut stream, &result)?;
                        }
                        Request::Set { key, value } => {
                            let result = match self.engine.set(key, value) {
                                Ok(()) => Response::Ok(None),
                                Err(e) => Response::StorageError(e.to_string()),
                            };
                            bincode::serialize_into(&mut stream, &result)?;
                        }
                        Request::Remove { key } => {
                            let result = match self.engine.remove(key) {
                                Ok(()) => Response::Ok(None),
                                Err(e) => Response::StorageError(e.to_string()),
                            };
                            bincode::serialize_into(&mut stream, &result)?;
                        }
                    }
                }
                Err(e) => {
                    warn!("get a incorrect tcp connection, msg{}", e.to_string());
                }
            }
        }
        Ok(())
    }
}
