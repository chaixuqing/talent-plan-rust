use crate::thread_pool::NaiveThreadPool;
use crate::KvsEngine;
use crate::{
    lib_rpc::{Request, Response},
    Result, ThreadPool,
};
use log::{info, warn};
use std::net;

const THREAD_COUNT: u32 = 8;

pub struct KvsServer<T: KvsEngine, TP: ThreadPool> {
    addr: net::SocketAddr,
    engine: T,
    thread_pool: TP,
}

impl<T: KvsEngine, TP: ThreadPool> KvsServer<T, TP> {
    pub fn new(addr: net::SocketAddr, engine: T, thread_pool: TP) -> KvsServer<T, TP> {
        KvsServer {
            addr,
            engine,
            thread_pool,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let listener = net::TcpListener::bind(self.addr)?;
        let thread_pool = NaiveThreadPool::new(THREAD_COUNT)?;
        // let mut thread_poll: Vec<thread::JoinHandle<()>> = Vec::new();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!(
                        "get a connection from {}",
                        stream.peer_addr().unwrap().to_string()
                    );
                    let engine_clone = self.engine.clone();
                    thread_pool.spawn(move || {
                        let request = bincode::deserialize_from(&mut stream).unwrap();
                        match request {
                            Request::Get { key } => {
                                let result = match engine_clone.get(key) {
                                    Ok(opt) => Response::Ok(opt),
                                    Err(e) => Response::StorageError(e.to_string()),
                                };
                                bincode::serialize_into(&mut stream, &result).unwrap();
                            }
                            Request::Set { key, value } => {
                                let result = match engine_clone.set(key, value) {
                                    Ok(()) => Response::Ok(None),
                                    Err(e) => {
                                        println!("get some error in engine set.");
                                        Response::StorageError(e.to_string())},
                                };
                                bincode::serialize_into(&mut stream, &result).unwrap();
                            }
                            Request::Remove { key } => {
                                let result = match engine_clone.remove(key) {
                                    Ok(()) => Response::Ok(None),
                                    Err(e) => Response::StorageError(e.to_string()),
                                };
                                bincode::serialize_into(&mut stream, &result).unwrap();
                            }
                        }
                    });
                }
                Err(e) => {
                    warn!("get a incorrect tcp connection, msg{}", e.to_string());
                }
            }
        }
        Ok(())
    }
}
