use kvs::{KvError, Result,KvsClient};
use std::{ net::SocketAddr};
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
    let mut client = KvsClient::new(args.addr)?;

    match args.command {
        Command::Set(command) => {
            let (key, value) = (command.key, command.value);
            return client.set(key, value)
        }
        Command::Get(command) => {
            let key = command.key;
            match client.get(key.to_owned())? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Command::Rm(command) => {
            let key = command.key;
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
        _ => {
            return Err(KvError::UnKnownCommandType);
        }
    }
    return Ok(());
}