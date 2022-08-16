use crate::error::Result;
use crate::lib_rpc::{Request, Response};
use std::net;

pub struct KvsClient {
    stream: net::TcpStream,
}

impl KvsClient {
    pub fn new(addr: net::SocketAddr) -> Result<KvsClient> {
        let stream = net::TcpStream::connect(addr)?;
        Ok(KvsClient { stream })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let request = Request::Get { key };
        let res = Result::from(self.get_response(request));
        res
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let request = Request::Set { key, value };
        Result::from(self.get_response(request))
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let request = Request::Remove { key };
        Result::from(self.get_response(request))
    }

    fn get_response(&mut self, request: Request) -> Response {
        bincode::serialize_into(&mut self.stream, &request).unwrap();
        bincode::deserialize_from(&mut self.stream).unwrap()
    }
}
