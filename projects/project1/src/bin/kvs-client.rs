use kvs::{KvError, KvStore, Result};
use std::{env::current_dir, net::SocketAddr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Set {
    pub key: String,
    pub value: String,
}

#[derive(Debug, StructOpt)]
pub struct Get {
    pub key: String,
}

#[derive(Debug, StructOpt)]
pub struct Rm {
    pub key: String,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "set")]
    Set(Set),
    #[structopt(name = "get")]
    Get(Get),
    #[structopt(name = "rm")]
    Rm(Rm),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "classify")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(
        long,
        value_name = "IP:PORT",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    addr: SocketAddr,
}

fn main() -> Result<()> {
    let args = ApplicationArguments::from_args();
    // println!("{:?}", args);
    let mut store = KvStore::open(current_dir().unwrap()).unwrap();
    match args.command {
        Command::Set(command) => {
            let (key, value) = (command.key, command.value);
            store.set(key.to_owned(), value.to_owned()).unwrap();
        }
        Command::Get(command) => {
            let key = command.key;
            match store.get(key.to_owned())? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Command::Rm(command) => {
            let key = command.key;
            if key.len() != 0 {
                return match store.remove(key.to_owned()) {
                    Ok(()) => Ok(()),
                    Err(KvError::KeyNotFound) => {
                        println!("{}", KvError::KeyNotFound);
                        Err(KvError::KeyNotFound)
                    }
                    Err(e) => return Err(e),
                };
            } else {
                println!("Key not found");
                return Err(KvError::KeyNotFound);
            }
        }
        _ => {
            return Err(KvError::UnKnownCommandType);
        }
    }
    return Ok(());
}