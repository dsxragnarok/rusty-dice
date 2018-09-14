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
    pub fn new(die: Die) -> Dice {
        Dice {
            die,
            sides: die as u32,
            n: 1,
            modifier: 0
        }
    }

    pub fn roll(&self) -> u32 {
        let range = Uniform::new_inclusive(1, self.sides);

        thread_rng().sample(range)
    }

    pub fn roll_n_times(&self, n: u32) -> Vec<u32> {
        let mut rolls = Vec::new();

        for _ in 0..n {
            rolls.push(self.roll());
        }

        rolls
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Dice) -> bool {
        self.sides == other.sides
    }
}

#[cfg(test)]
mod tests {
    use super::Dice;
    use super::Die;

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
    fn it_returns_number_between_1_and_4() {
        for _ in 0..100 {
            let d4 = Dice::new(Die::D4);
            let roll = d4.roll();
            assert!(roll >= 1 && roll <= 4);
        }
    }

    #[test]
    fn it_returns_number_vector_of_rolls_within_range() {
        let d6 = Dice::new(Die::D6);
        let rolls = d6.roll_n_times(30);

        assert_eq!(rolls.len(), 30);

        for roll in rolls.iter() {
            assert!(*roll >= 1 && *roll <= 6);
        }
    }

}
