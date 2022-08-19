use criterion::*;
use kvs::KvStore;
use kvs::KvsEngine;
use kvs::SledKvsEngine;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::tempdir;

const MAX_DATA_LEN: usize = 1000;

fn gen_set_data(size: usize) -> Vec<(String, String)> {
    let mut set_data = Vec::with_capacity(size);
    let mut rng = thread_rng();
    for _ in 0..size {
        let len = rng.gen_range(1, MAX_DATA_LEN);
        let key: String = rng.sample_iter(&Alphanumeric).take(len).collect();
        let len = rng.gen_range(1, MAX_DATA_LEN);
        let value: String = rng.sample_iter(&Alphanumeric).take(len).collect();
        set_data.push((key, value));
    }
    set_data
}

fn gen_get_data(size: usize, set_data: &Vec<(String, String)>) -> Vec<String> {
    let mut get_data: Vec<String> = Vec::with_capacity(size);
    let len = set_data.len();
    let mut rng = thread_rng();
    for _ in 0..size {
        let index = rng.gen_range(0, len);
        get_data.push(set_data[index].0.clone());
    }
    get_data
}

fn kvstore_bench_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("kvstore_bench_write");
    for size in [20, 40, 60, 80, 100] {
        let set_data = gen_set_data(size);
        group.bench_with_input(BenchmarkId::from_parameter(&size), &size, |b, &_size| {
            b.iter(|| {
                let temp_dir = tempdir().unwrap();
                let mut kv_engine = KvStore::open(temp_dir.path()).unwrap();
                for (key, value) in set_data.iter() {
                    kv_engine.set(key.to_owned(), value.to_owned()).unwrap();
                }
            });
        });
    }
    group.finish();
}

fn sled_bench_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("sled_bench_write");
    for size in [20, 40, 60, 80, 100] {
        let set_data = gen_set_data(size);
        group.bench_with_input(BenchmarkId::from_parameter(&size), &size, |b, &_size| {
            b.iter(|| {
                let mut kv_engine = SledKvsEngine::new(tempdir().unwrap().path());
                for (key, value) in set_data.iter() {
                    kv_engine.set(key.to_owned(), value.to_owned()).unwrap();
                }
            });
        });
    }
    group.finish();
}

fn sled_bench_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("sled_bench_read");
    for size in [20, 40, 60, 80, 100] {
        let set_data = gen_set_data(size);
        let mut kv_engine = SledKvsEngine::new(tempdir().unwrap().path());
        for (key, value) in set_data.iter() {
            kv_engine.set(key.to_owned(), value.to_owned()).unwrap();
        }
        let get_data = gen_get_data(size / 2, &set_data);
        group.bench_with_input(BenchmarkId::from_parameter(&size), &size, |b, &_size| {
            b.iter(|| {
                for key in get_data.iter() {
                    kv_engine.get(key.to_owned()).unwrap();
                }
            });
        });
    }
    group.finish();
}

fn kvstore_bench_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("kvstore_bench_read");
    for size in [20, 40, 60, 80, 100] {
        let set_data = gen_set_data(size);
        let mut kv_engine = KvStore::open(tempdir().unwrap().path()).unwrap();
        for (key, value) in set_data.iter() {
            kv_engine.set(key.to_owned(), value.to_owned()).unwrap();
        }
        let get_data = gen_get_data(size / 2, &set_data);
        group.bench_with_input(BenchmarkId::from_parameter(&size), &size, |b, &_size| {
            b.iter(|| {
                for key in get_data.iter() {
                    kv_engine.get(key.to_owned()).unwrap();
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    kvstore_bench_write,
    sled_bench_write,
    sled_bench_read,
    kvstore_bench_read
);
criterion_main!(benches);
