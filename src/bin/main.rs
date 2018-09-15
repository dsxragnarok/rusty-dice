extern crate rustydice;

use rustydice::dice::{Die, Roll};
use rustydice::logger;

fn main() {
    let roll = Roll::new(Die::D8).number_of_rolls(2).modifier(3).roll();
    let rolls  = &roll.rolls;
    println!("{}", logger::build_log(&roll));
    println!(" >>> {:?}", rolls);

    let roll = Roll::new(Die::D4).number_of_rolls(4).roll();
    let rolls  = &roll.rolls;
    println!("{}", logger::build_log(&roll));
    println!(" >>> {:?}", rolls);


    let d = "d4";
    let d: Vec<&str> = d.split('d').collect();
    println!("{:?}", d);
}
