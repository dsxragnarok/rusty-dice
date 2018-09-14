extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use std::any::{Any, TypeId};

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
}

impl PartialEq for Dice {
    fn eq(&self, other: &Dice) -> bool {
        self.sides == other.sides
    }
}

struct RollResult {
    die: Die,
    rolls: Vec<i32>,
    modifier: i32,
    total: i32
}

trait InstanceOf
where
    Self: Any,
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}
impl<RollResult: ?Sized + Any> InstanceOf for RollResult {}

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
    fn it_returns_RollResult() {
        let d6 = Dice::new(Die::D6);
        let result = d6.roll();

        assert!(result.instance_of::<RollResult>());
    }
}
