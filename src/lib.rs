extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

#[derive(Debug, Copy, Clone)]
pub enum Die {
    D2 = 2,
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100
}

#[derive(Debug)]
pub struct Dice {
    die: Die,
    sides: u32,
    n: u32,
    modifier: i32
}

impl Dice {
    fn _roll(&self) -> u32 {
        let range = Uniform::new_inclusive(1, self.sides);

        thread_rng().sample(range)
    }

    pub fn new(die: Die) -> Dice {
        Dice {
            die,
            sides: die as u32,
            n: 1,
            modifier: 0
        }
    }

    pub fn n(&mut self, n: u32) {
        self.n = n;
    }

    pub fn modifier(&mut self, modifier: i32) {
        self.modifier = modifier;
    }

    pub fn roll(&mut self) -> RollResult {
        let mut rolls = Vec::new();
        let mut sum = 0;
        for _ in 0..self.n {
            let roll = self._roll();
            rolls.push(roll);
            sum += roll;
        }

        RollResult {
            die: self.die,
            modifier: self.modifier,
            total: sum as i32 + self.modifier,
            rolls,
        }
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Dice) -> bool {
        self.sides == other.sides
    }
}

pub struct RollResult {
    die: Die,
    rolls: Vec<u32>,
    modifier: i32,
    total: i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_four_sided_die() {
        let d4 = Dice::new(Die::D4);
        assert_eq!(d4.die as u32, 4);
    }
    #[test]
    fn it_creates_a_eight_sided_die() {
        let d8 = Dice::new(Die::D8);
        assert_eq!(d8.die as u32, 8);
    }
    #[test]
    fn it_creates_a_twenty_sided_die() {
        let d20 = Dice::new(Die::D20);
        assert_eq!(d20.die as u32, 20);
    }

    #[test]
    fn it_sets_the_n_property() {
        let mut die = Dice::new(Die::D12);
        die.n(3);

        assert_eq!(die.n, 3);
    }

    #[test]
    fn it_sets_the_modifier_property() {
        let mut die = Dice::new(Die::D100);
        die.modifier(-20);

        assert_eq!(die.modifier, -20);
    }

    #[test]
    fn it_returns_number_between_1_and_4() {
        for _ in 0..100 {
            let d4 = Dice::new(Die::D4);
            let roll = d4._roll();
            assert!(roll >= 1 && roll <= 4);
        }
    }

    #[test]
    fn it_returns_resulting_vector_of_rolls_and_total() {
        let mut d6 = Dice::new(Die::D6);
        d6.n(6);
        d6.modifier(3);
        let result = d6.roll();

        assert_eq!(result.die as u32, d6.die as u32);
        assert_eq!(result.modifier, d6.modifier);
        assert_eq!(result.rolls.len(), d6.n as usize);

        let sum: u32 = result.rolls.iter().sum();
        assert_eq!(result.total, sum as i32 + result.modifier);
    }
}
