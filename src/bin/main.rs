extern crate rustydice;
extern crate chrono;

use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::Write;
use chrono::Local;
use rustydice::command::Command;
use rustydice::logger;

static USAGE: &str = "USAGE:\trustydice ndx[+|-]m";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-i")) || args.contains(&String::from("--interactive")) {
        interactive_mode(args.get(2));
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

fn interactive_mode(logfile: Option<&String>) {
    display_help();

    let ref mut file_result = match logfile {
        Some(path) => Ok(File::create(path).expect("Unable to create file")),
        None => Err("No file specified"),
    };

    loop {
        println!("Enter your roll command: ndx[+|-]m");

        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("failed to read input");

        match query.trim() {
            "help" => display_help(),
            "exit" => process::exit(0),
            _ => {
                let log = execute_roll(&query);

                match file_result {
                    Ok(file) => file.write_all(log.as_bytes()).expect("Unable to write log"),
                    Err(_) => () // If there is no filepath specified then no log is written
                }
            },
        }
    }
}

fn display_help() {
    println!("exit: quit program");
    println!("help: prints this help text");
    println!("Roll Syntax: ndx[+|-]m");
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

fn execute_roll(query: &String) -> String {
    let roll = Command::from(&query.trim()[..]).run();
    let log = logger::build_log(&roll);

    let dt = Local::now().format("%m/%d/%Y %H:%M:%S");

    let log = format!("[{}] {}\n >>> {:?}\n", dt, log, &roll.rolls);

    println!("{}", log);

    log
}
