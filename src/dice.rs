use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

/// The available dice types denoting the number of sides
#[derive(Debug, Copy, Clone, PartialEq)]
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

impl From<u32> for Die {
    fn from(sides: u32) -> Die {
        return match sides {
            2 => Die::D2,
            4 => Die::D4,
            6 => Die::D6,
            8 => Die::D8,
            10 => Die::D10,
            12 => Die::D12,
            20 => Die::D20,
            100 => Die::D100,
            _ => panic!("Unable to match the die type"),
        }
    }
}

impl From<Die> for u32 {
    fn from(die: Die) -> u32 {
        die as u32
    }
}

/// Represents a Roll Roll
#[derive(Debug, PartialEq)]
pub struct Roll {
    die: Die,
    modifier: i32,
    number_of_rolls: u32,
}

impl Roll {
    /// Returns a random number between 1 and maximum number on a given die
    fn _roll(&self) -> u32 {
        let range = Uniform::new_inclusive(1, self.die.into());

        thread_rng().sample(range)
    }

    /// Returns a Roll instance
    ///
    /// # Parameters
    /// * `die` - The die type
    pub fn new(die: Die) -> Roll {
        Roll {
            die,
            number_of_rolls: 1,
            modifier: 0,
        }
    }

    /// Sets the `number_of_rolls` property and returns **Self** to allow for chaining
    ///
    /// # Parameters
    /// * `n` - The number of times to roll the die
    pub fn number_of_rolls(&mut self, n: u32) -> &mut Roll {
        self.number_of_rolls = n;
        self
    }

    /// Sets the `modifier` property and returns **Self** to allow for chaining
    ///
    /// # Parameters
    /// * `m` - The modifier to the sum result
    pub fn modifier(&mut self, m: i32) -> &mut Roll {
        self.modifier = m;
        self
    }

    /// Executes the rolls and returns the RollResult
    pub fn roll(&mut self) -> RollResult {
        let mut rolls = Vec::new();
        let mut sum = 0;
        for _ in 0..self.number_of_rolls {
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

impl<'a> From<&'a str> for Roll {
    fn from(text: &str) -> Roll {
        let string_parts: Vec<&str> = text.split('d').collect();
        let n = string_parts[0];
        let number_of_rolls: u32 = if n == "" { 1 } else { n.parse().unwrap() };

        let predicate = string_parts[1];
        let index = match predicate.find('+') {
            Some(i) => i,
            None => match predicate.find('-') {
                Some(i) => i,
                None => 0,
            },
        };

        let (die, modifier) = if index > 0 {
            (
                &predicate[..index],
                predicate[index..].parse::<i32>().unwrap()
            )
        } else {
            (predicate, 0)
        };

        let die = match die {
            "2" => Die::D2,
            "4" => Die::D4,
            "6" => Die::D6,
            "8" => Die::D8,
            "10" => Die::D10,
            "12" => Die::D12,
            "20" => Die::D20,
            "100" => Die::D100,
            _ => panic!("Unable to match the die type!"),
        };

        Roll {
            die,
            number_of_rolls,
            modifier,
        }
    }
}

/// Represents the result of a roll session
#[derive(Debug)]
pub struct RollResult {
    pub die: Die,
    pub rolls: Vec<u32>,
    pub modifier: i32,
    pub total: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_four_sided_die() {
        let d4 = Roll::new(Die::D4);
        assert_eq!(d4.die as u32, 4);
    }
    #[test]
    fn it_creates_a_eight_sided_die() {
        let d8 = Roll::new(Die::D8);
        assert_eq!(d8.die as u32, 8);
    }
    #[test]
    fn it_creates_a_twenty_sided_die() {
        let d20 = Roll::new(Die::D20);
        assert_eq!(d20.die as u32, 20);
    }

    #[test]
    fn it_sets_the_n_property() {
        let mut die = Roll::new(Die::D12);
        die.number_of_rolls(3);

        assert_eq!(die.number_of_rolls, 3);
    }

    #[test]
    fn it_sets_the_modifier_property() {
        let mut die = Roll::new(Die::D100);
        die.modifier(-20);

        assert_eq!(die.modifier, -20);
    }

    #[test]
    fn it_returns_number_between_1_and_4() {
        for _ in 0..100 {
            let d4 = Roll::new(Die::D4);
            let roll = d4._roll();
            assert!(roll >= 1 && roll <= 4);
        }
    }

    #[test]
    fn it_returns_resulting_vector_of_rolls_and_total() {
        let mut d6 = Roll::new(Die::D6);
        d6.number_of_rolls(6);
        d6.modifier(3);
        let result = d6.roll();

        assert_eq!(result.die as u32, d6.die as u32);
        assert_eq!(result.modifier, d6.modifier);
        assert_eq!(result.rolls.len(), d6.number_of_rolls as usize);

        let sum: u32 = result.rolls.iter().sum();
        assert_eq!(result.total, sum as i32 + result.modifier);
    }

    #[test]
    fn it_should_chain_methods_to_build_roll_result() {
        let result = Roll::new(Die::D10).number_of_rolls(9).modifier(4).roll();

        assert_eq!(result.die as u32, Die::D10 as u32);
        assert_eq!(result.modifier, 4);
        assert_eq!(result.rolls.len(), 9);

        let sum: u32 = result.rolls.iter().sum();
        assert_eq!(result.total, sum as i32 + result.modifier);
    }

    #[test]
    fn it_parses_a_unit_d4_die() {
        let expected = Roll::new(Die::D4);
        let roll = Roll::from("d4");

        assert_eq!(expected, roll);
    }

    #[test]
    fn it_parses_a_unit_d12_die() {
        let expected = Roll::new(Die::D12);
        let roll = Roll::from("d12");

        assert_eq!(expected, roll);
    }

    #[test]
    fn it_parses_2d8() {
        let mut expected = Roll::new(Die::D8);
        expected.number_of_rolls(2);

        let roll = Roll::from("2d8");

        assert_eq!(expected, roll);
    }

    #[test]
    fn it_parses_3d6_with_positive_modifier() {
        let mut expected = Roll::new(Die::D6);
        expected.number_of_rolls(3).modifier(2);

        let roll = Roll::from("3d6+2");

        assert_eq!(expected, roll);
    }

    #[test]
    fn it_parses_1d20_with_negative_modifier() {
        let mut expected = Roll::new(Die::D20);
        expected.modifier(-4);

        let roll = Roll::from("1d20-4");

        assert_eq!(expected, roll);
    }
}
