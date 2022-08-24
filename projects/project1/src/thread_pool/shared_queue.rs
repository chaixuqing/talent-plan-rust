use super::ThreadPool;
use crate::Result;
use core::time;
use crossbeam_channel::{unbounded, Receiver, Sender};
use defer::defer;
use num_cpus;
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum ThreadPoolMessage {
    RunJob(Job),
    Shutdown,
}

pub enum ControlMessage {
    Start,
    Stop,
}

pub struct SharedQueueThreadPool {
    size: usize,
    thread_pool_sx: Sender<ThreadPoolMessage>,
    monitor_sx: Sender<ControlMessage>,
    monitor_handle: JoinHandle<()>,
}

pub struct Monitor {
    thread_pool_rx: Receiver<ThreadPoolMessage>,
    monitor_rx: Receiver<ControlMessage>,
    prx: Receiver<usize>,
    psx: Sender<usize>,
    threads: Vec<Worker>,
}

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<SharedQueueThreadPool> {
        let mut size = threads as usize;
        if size == 0 {
            size = num_cpus::get();
        }
        let (thread_pool_sx, thread_pool_rx) = unbounded();
        let (monitor_sx, monitor_rx) = unbounded();
        let monitor_handle = thread::spawn(move || {
            let mut monitor = Monitor::new(size, thread_pool_rx, monitor_rx);
            monitor.watch();
        });
        Ok(SharedQueueThreadPool {
            size,
            thread_pool_sx,
            monitor_sx,
            monitor_handle,
        })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.thread_pool_sx
            .send(ThreadPoolMessage::RunJob(Box::new(job)))
            .unwrap();
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        println!("dropped SharedQueueThreadPool");
        self.monitor_sx.send(ControlMessage::Stop).unwrap();
        for _ in 0..self.size {
            self.thread_pool_sx
                .send(ThreadPoolMessage::Shutdown)
                .unwrap();
        }
    }
}

impl Monitor {
    pub fn new(
        size: usize,
        thread_pool_rx: Receiver<ThreadPoolMessage>,
        monitor_rx: Receiver<ControlMessage>,
    ) -> Monitor {
        let (psx, prx) = unbounded();
        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i, thread_pool_rx.clone(), psx.clone()));
        }
        Monitor {
            thread_pool_rx,
            monitor_rx,
            prx,
            psx,
            threads,
        }
    }

    pub fn watch(&mut self) {
        loop {
            if let Ok(id) = self.prx.try_recv() {
                println!("create a new thread.");
                self.threads[id] = Worker::new(id, self.thread_pool_rx.clone(), self.psx.clone());
            }
            if let Ok(ControlMessage::Stop) = self.monitor_rx.try_recv() {
                break;
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}

impl Drop for Monitor{
    fn drop(&mut self) {
        println!("dropped Monitor.");
    }
}

impl Worker {
    pub fn new(id: usize, rx: Receiver<ThreadPoolMessage>, psx: Sender<usize>) -> Worker {
        let handle = thread::spawn(move || {
            while let Ok(ThreadPoolMessage::RunJob(job)) = rx.recv() {
                job()
            }
            psx.send(id);
        });
        Worker { id, handle }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        eprintln!("dropped worker {}", self.id);
    }
}
