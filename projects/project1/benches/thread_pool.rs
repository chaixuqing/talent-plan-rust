use std::{net::SocketAddr, str::FromStr, thread, time::Duration};

use assert_cmd::assert;
use criterion::*;
use crossbeam_utils::sync::WaitGroup;
use kvs::{
    thread_pool::SharedQueueThreadPool, ArcKvStore, KvsClient, KvsServer, SledKvsEngine, ThreadPool,
};
use num_cpus;
use tempfile::tempdir;

const KEY_COUNT: u32 = 100;

fn read_queue_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_queue_kvstore");

    let mut inputs = Vec::new();
    for i in 1..=(num_cpus::get() * 2) {
        inputs.push(i);
    }
    for size in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let addr = SocketAddr::from_str("127.0.0.1:8888").unwrap();
            let dir = tempdir().unwrap();
            let engine = ArcKvStore::open(dir.path()).unwrap();
            let thread_pool = SharedQueueThreadPool::new(size as u32).unwrap();
            let mut server = KvsServer::new(addr, engine, thread_pool);
            let server_handle = thread::spawn(move || {
                server.start();
            });
            thread::sleep(Duration::from_secs(1));

            let keys: Vec<String> = (0..KEY_COUNT).map(|x| format!("key_{}", x)).collect();
            let value = String::from("value");

            for key in keys.iter() {
                KvsClient::new(addr)
                    .unwrap()
                    .set(key.clone(), value.clone())
                    .unwrap();
            }

            let client_thread_pool = SharedQueueThreadPool::new(KEY_COUNT).unwrap();
            b.iter(|| {
                let wg = WaitGroup::new();
                let keys = keys.clone();
                let addr = addr.clone();
                for key in keys {
                    let wg = wg.clone();
                    let value = value.clone();
                    client_thread_pool.spawn(move || {
                        let res = KvsClient::new(addr).unwrap().get(key).unwrap().unwrap();
                        assert_eq!(res, value);
                    });
                    drop(wg);
                }
                wg.wait();
            });

            drop(server_handle);
        });
    }
    group.finish();
}

fn write_queue_kvstore(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_queue_kvstore");

    let mut inputs = Vec::new();
    for i in 1..=(num_cpus::get() * 2) {
        inputs.push(i);
    }
    for size in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let addr = SocketAddr::from_str("127.0.0.1:8888").unwrap();
            let dir = tempdir().unwrap();
            let engine = ArcKvStore::open(dir.path()).unwrap();
            let thread_pool = SharedQueueThreadPool::new(size as u32).unwrap();
            let mut server = KvsServer::new(addr, engine, thread_pool);
            let server_handle = thread::spawn(move || {
                server.start();
            });
            thread::sleep(Duration::from_secs(1));

            let keys: Vec<String> = (0..KEY_COUNT).map(|x| format!("key_{}", x)).collect();
            let value = String::from("value");

            let client_thread_pool = SharedQueueThreadPool::new(KEY_COUNT).unwrap();
            b.iter(|| {
                let wg = WaitGroup::new();
                let keys = keys.clone();
                let addr = addr.clone();
                for key in keys {
                    let wg = wg.clone();
                    let value = value.clone();
                    client_thread_pool.spawn(move || {
                        KvsClient::new(addr)
                            .unwrap()
                            .set(key.clone(), value.clone())
                            .unwrap();
                    });
                    drop(wg);
                }
                wg.wait();
            });

            drop(server_handle);
        });
    }
    group.finish();
}

criterion_group!(benches, read_queue_kvstore, write_queue_kvstore);
criterion_main!(benches);
