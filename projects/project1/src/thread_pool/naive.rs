use std::thread;

use crate::thread_pool::ThreadPool;
use super::super::Result;
pub struct NaiveThreadPool {}

impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> Result<NaiveThreadPool>{
        Ok(NaiveThreadPool{})
    }
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static{
        thread::spawn(job);
    }
}

impl NaiveThreadPool {
    pub fn some() {
        
    }
}