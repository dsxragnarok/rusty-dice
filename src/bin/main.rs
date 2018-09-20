extern crate rustydice;

use std::env;
use std::process;
use std::io;
use rustydice::command::Command;
use rustydice::logger;

static USAGE: &str = "USAGE:\trustydice ndx[+|-]m";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-i")) || args.contains(&String::from("--interactive")) {
        interactive_mode();
    } else {
        single_process_mode(env::args());
    }
}

fn single_process_mode(ref mut args: env::Args) {
    args.next(); // skip the first argument as that is the program

    let query = match args.next() {
        Some(arg) => arg,
        None => {
            println!("{}", USAGE);
            process::exit(1);
        },
    };

    execute_roll(&query);
}

fn interactive_mode() {
    display_help();

    loop {
        println!("Enter your roll command: ndx[+|-]m");

        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("failed to read input");

        match query.trim() {
            "help" => display_help(),
            _ => execute_roll(&query),
        }
    }
}

fn display_help() {
    println!("Command Syntax: ndx[+|-]m");
    println!("    [n]: number of times to roll the die");
    println!("[+m|-m]: the modifier to apply to the sum result");
    println!("   [dx]: the type of die to roll");
    println!("\t  [d2]: 2-sided die");
    println!("\t  [d4]: 4-sided die");
    println!("\t  [d6]: 6-sided die");
    println!("\t  [d8]: 8-sided die");
    println!("\t [d10]: 10-sided die");
    println!("\t [d12]: 12-sided die");
    println!("\t [d20]: 20-sided die");
    println!("\t[d100]: 100-sided die");
}

fn execute_roll(query: &String) {
    let roll = Command::from(&query.trim()[..]).run();
    let log = logger::build_log(&roll);
    println!("{}", log);
    println!(" >>> {:?}", &roll.rolls);
}
