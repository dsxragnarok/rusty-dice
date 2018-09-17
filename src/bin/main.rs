extern crate rustydice;

use std::env;
use std::process;
use rustydice::command::Command;
use rustydice::logger;

static USAGE: &str = "USAGE:\trustydice ndx[+|-]m";

fn main() {
    let mut args = env::args();
    args.next();

    let query = match args.next() {
        Some(arg) => arg,
        None => {
            println!("{}", USAGE);
            process::exit(1);
        },
    };

    let roll = Command::from(&query[..]).run();
    let log = logger::build_log(&roll);
    println!("{}", log);
    println!(" >>> {:?}", &roll.rolls);
}
