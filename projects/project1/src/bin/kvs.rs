use clap::{crate_version, App, Arg, SubCommand};
use kvs::{KvError, KvStore, Result};
use std::{env::current_dir};

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(crate_version!())
        .author("CARGO_PKG_AUTHORS ")
        .about("CARGO_PKG_DESCRIPTION")
        .subcommand(
            SubCommand::with_name("get")
                .about("this is get about things")
                .arg(Arg::with_name("key").help("key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("this is set about things")
                .arg(Arg::with_name("key").help("key").required(true))
                .arg(Arg::with_name("value").help("value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("this is rm about things")
                .arg(Arg::with_name("key").help("key").required(true)),
        )
        .get_matches();
    let mut store = KvStore::open(current_dir().unwrap()).unwrap();
    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            match store.get(key.to_owned())? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
            Ok(())
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let val = matches.value_of("value").unwrap();
            store.set(key.to_owned(), val.to_owned()).unwrap();
            Ok(())
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("key");
            match key {
                Some(s) => {
                    match store.remove(s.to_owned()) {
                        Ok(()) => {Ok(())},
                        Err(KvError::KeyNotFound) => {
                            println!("{}", KvError::KeyNotFound);
                            Err(KvError::KeyNotFound)
                        }
                        Err(e) => return Err(e),
                    }
                }
                None => {
                    println!("Key not found");
                    Err(KvError::KeyNotFound)
                }
            }
        }
        _ => unreachable!(),
    }
}
