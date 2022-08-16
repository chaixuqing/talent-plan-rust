use kvs::{KvError, KvsClient, Result};
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Set {
    pub key: String,
    pub value: String,

    #[structopt(
        long,
        value_name = "IP:PORT",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    addr: SocketAddr,
}

#[derive(Debug, StructOpt)]
pub struct Get {
    pub key: String,

    #[structopt(
        long,
        value_name = "IP:PORT",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    addr: SocketAddr,
}

#[derive(Debug, StructOpt)]
pub struct Rm {
    pub key: String,

    #[structopt(
        long,
        value_name = "IP:PORT",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    addr: SocketAddr,
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
}

fn main() -> Result<()> {
    let args = ApplicationArguments::from_args();

    match args.command {
        Command::Set(command) => {
            let (key, value) = (command.key, command.value);
            let mut client = KvsClient::new(command.addr)?;
            return client.set(key, value);
        }
        Command::Get(command) => {
            let key = command.key;
            let mut client = KvsClient::new(command.addr)?;
            match client.get(key.to_owned())? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Command::Rm(command) => {
            let key = command.key;
            let mut client = KvsClient::new(command.addr)?;
            if key.len() != 0 {
                return match client.remove(key.to_owned()) {
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
    }
    return Ok(());
}
