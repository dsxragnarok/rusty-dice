extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

struct Dice {
    sides: u32
}

impl Dice {
    pub fn new(sides: u32) -> Dice {
        Dice { sides }
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

#[cfg(test)]
mod tests {

    #[test]
    fn it_returns_number_between_1_and_4() {
        for _ in 0..100 {
            let d4 = super::Dice::new(4);
            let roll = d4.roll();
            assert!(roll >= 1 && roll <= 4);
        }
    }

    #[test]
    fn it_returns_number_vector_of_rolls_within_range() {
        let d6 = super::Dice::new(6);
        let rolls = d6.roll_n_times(30);

        assert_eq!(rolls.len(), 30);

        for roll in rolls.iter() {
            assert!(*roll >= 1 && *roll <= 6);
        }
    }
}
