mod naive;
mod shared_queue;
mod rayon;

pub use naive::NaiveThreadPool;
pub use shared_queue::SharedQueueThreadPool;
pub use self::rayon::RayonThreadPool;
use super::Result;
pub trait ThreadPool {
    fn new(threads: u32) -> Result<Self> where Self: Sized;
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}