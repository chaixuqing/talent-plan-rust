use clap::arg_enum;
use kvs::{ArcKvStore, KvError, KvsEngine, KvsServer, Result, SledKvsEngine};
use log::{error, info, LevelFilter};
use std::{env::current_dir, fs, net::SocketAddr};
use structopt::StructOpt;

const DEFAULT_IP_ADDR: &str = "127.0.0.1:4000";
const ENGINE_CONFIGURE_FILE_NAME: &str = "engineConfigure.txt";

arg_enum! {
    #[derive(Debug,PartialEq)]
    enum EngineType {
        Kvs,
        Sled
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(
        long,
        value_name = "IP:PORT",
        default_value = DEFAULT_IP_ADDR,
        parse(try_from_str)
    )]
    addr: SocketAddr,

    #[structopt(long,value_name = "ENGINE-NAME",possible_values = &EngineType::variants(),case_insensitive = true, parse(try_from_str))]
    engine: Option<EngineType>,
}

fn get_prev_engine() -> Result<Option<EngineType>> {
    let path = current_dir().unwrap().join(ENGINE_CONFIGURE_FILE_NAME);
    if !path.exists() {
        info!("path is not exist.");
        return Ok(None);
    }
    match &fs::read_to_string(path)?[..] {
        "Kvs" => Ok(Some(EngineType::Kvs)),
        "Sled" => Ok(Some(EngineType::Sled)),
        _ => {
            info!("{} has some error context", ENGINE_CONFIGURE_FILE_NAME);
            Err(KvError::UnKnownEngineType)
        }
    }
}

fn get_engine(engine: Option<EngineType>) -> Result<EngineType> {
    info!("call get_engine.");
    let prev_engine = get_prev_engine()?;
    if engine == None {
        info!("engine is None.");
        if prev_engine == None {
            return Ok(EngineType::Kvs);
        } else {
            return Ok(prev_engine.unwrap());
        }
    } else {
        if prev_engine == None || prev_engine == engine {
            return Ok(engine.unwrap());
        } else {
            error!("prev engine isn't match to the engine in args.");
            return Err(KvError::UnKnownEngineType);
        }
    }
}

fn init_engine_configure(engine: &EngineType) {
    let path = current_dir().unwrap().join(ENGINE_CONFIGURE_FILE_NAME);
    if !path.exists() {
        fs::File::create(&path).unwrap();
    }
    fs::write(path, engine.to_string()).unwrap();
}

fn main() {
    let opt = Opt::from_args();
    println!("args:{:?},engine:{:?}", opt.addr, opt.engine);
    let mut builder = env_logger::Builder::new();

    // builder.format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()));

    builder.filter_level(LevelFilter::Info).init();
    let engine = get_engine(opt.engine).unwrap();
    init_engine_configure(&engine);
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Storage Engine: {:?}", engine);
    info!("Socket Address: {}", opt.addr);

    match engine {
        EngineType::Kvs => {
            info!("start kvStore engine.");
            start_engine(opt.addr, ArcKvStore::new(current_dir().unwrap()));
        }
        EngineType::Sled => {
            info!("start sled engine.");
            start_engine(
                opt.addr,
                SledKvsEngine::new(current_dir().unwrap().as_path()),
            );
        }
    };
}

fn start_engine(addr: SocketAddr, kv_engine: impl KvsEngine) {
    let mut server = KvsServer::new(addr, kv_engine);
    server.start().unwrap()
}
