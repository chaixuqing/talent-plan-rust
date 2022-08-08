use clap::arg_enum;
use kvs::{KvError, Result};
use log::{info, LevelFilter};
use std::{env::current_dir, fs, net::SocketAddr,io::Write};
use structopt::StructOpt;

const DEFAULT_IP_ADDR: &str = " ";
const ENGINE_CONFIGURE_FILE_NAME: &str = "engineConfigure.txt";

arg_enum! {
    #[derive(Debug,PartialEq)]
    enum EngineType {
        kvs,
        sled
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
        return Ok(None);
    }
    match &fs::read_to_string(path)?[..] {
        "kvs" => Ok(Some(EngineType::kvs)),
        "sled" => Ok(Some(EngineType::sled)),
        _ => Err(KvError::UnKnownEngineType),
    }
}

fn get_engine(engine: Option<EngineType>) -> Result<EngineType> {
    let prev_engine = get_prev_engine()?;
    if engine == None {
        if prev_engine == None {
            return Ok(EngineType::kvs);
        } else {
            return Ok(prev_engine.unwrap());
        }
    } else {
        if prev_engine == None || prev_engine == engine {
            return Ok(engine.unwrap());
        } else {
            println!("here");
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

    builder.format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()));

    builder.filter_level(LevelFilter::Debug)
        .init();
    let engine = get_engine(opt.engine).unwrap();
    init_engine_configure(&engine);
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Storage Engine: {:?}", engine);
    info!("Socket Address: {}", opt.addr);
}
