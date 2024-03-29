mod naive;
mod rayon;
mod shared_queue;

pub use self::rayon::RayonThreadPool;
use super::Result;
pub use naive::NaiveThreadPool;
pub use shared_queue::SharedQueueThreadPool;
pub trait ThreadPool: Send + 'static {
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}