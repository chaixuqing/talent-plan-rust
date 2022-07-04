use clap::{crate_version, App, Arg, SubCommand};
use std::process::exit;

fn main() {
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
    match matches.subcommand() {
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(_matches)) => {
            // if matches.is_present("key") && matches.is_present("value") {
            //     println!("{}", matches.value_of("key").unwrap());
            //     println!("{}", matches.value_of("value").unwrap());
            // }
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            // if matches.is_present("key") {
            //     println!("{}", matches.value_of("key").unwrap());
            // }
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
