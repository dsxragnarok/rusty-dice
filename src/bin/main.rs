extern crate rustydice;

use std::env;
use std::process;
use rustydice::command;
use rustydice::dice;
use rustydice::logger;

static USAGE: &str = "USAGE:\trusty-dice ndx[+|-]m";

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

    let cmd = command::parse(&query[..]);

    // need to implement a run() from Command struct
    let roll = dice::Roll::new(cmd.die)
                    .number_of_rolls(cmd.number_of_rolls)
                    .modifier(cmd.modifier)
                    .roll();

    let log = logger::build_log(&roll);
    println!("{}", log);
    println!(" >>> {:?}", &roll.rolls);
}
