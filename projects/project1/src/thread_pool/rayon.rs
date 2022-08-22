use rayon;

use crate::{ThreadPool,Result};

pub struct RayonThreadPool(rayon::ThreadPool);

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self>{
       Ok(RayonThreadPool(rayon::ThreadPoolBuilder::new().num_threads(threads as usize).build().unwrap())) 
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        self.0.spawn(job);
    }
}