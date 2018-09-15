extern crate rustydice;

use rustydice::dice::{Die, Roll};
use rustydice::logger;

fn main() {
    let roll = Roll::new(Die::D8).number_of_rolls(2).modifier(3).roll();
    println!("{}",  logger::build_log(roll));

    let roll = Roll::new(Die::D4).number_of_rolls(4).roll();
    println!("{}",  logger::build_log(roll));
}
