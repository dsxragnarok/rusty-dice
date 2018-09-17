use dice::{Die, Roll, RollResult};
use dice::Die::*;

#[derive(Debug)]
pub struct Command {
    pub number_of_rolls: u32,
    pub die: Die,
    pub modifier: i32,
}

impl Command {
    pub fn new(number_of_rolls: u32, die: Die, modifier: i32) -> Command {
        Command {
            die,
            modifier,
            number_of_rolls,
        }
    }

    pub fn from(input_text: &str) -> Command {
        let string_parts: Vec<&str> = input_text.split('d').collect();
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
            "2" => D2,
            "4" => D4,
            "6" => D6,
            "8" => D8,
            "10" => D10,
            "12" => D12,
            "20" => D20,
            "100" => D100,
            _ => panic!("Unable to match the die type!"),
        };

        Command::new(number_of_rolls, die, modifier)
    }

    pub fn run(&self) -> RollResult {
        Roll::new(self.die)
            .number_of_rolls(self.number_of_rolls)
            .modifier(self.modifier)
            .roll()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_command_is_valid(expected: Command, actual: Command) {
        assert_eq!(expected.number_of_rolls, actual.number_of_rolls);
        assert_eq!(expected.modifier, actual.modifier);
        assert_eq!(expected.die as u32, actual.die as u32);
    }

    #[test]
    fn it_parses_a_unit_d4_die() {
        let expected = Command::new(1, D4, 0);
        let cmd = Command::from("d4");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d6_die() {
        let expected = Command::new(1, D6, 0);
        let cmd = Command::from("d6");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d8_die() {
        let expected = Command::new(1, D8, 0);
        let cmd = Command::from("d8");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d10_die() {
        let expected = Command::new(1, D10, 0);
        let cmd = Command::from("d10");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d12_die() {
        let expected = Command::new(1, D12, 0);
        let cmd = Command::from("d12");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d20_die() {
        let expected = Command::new(1, D20, 0);
        let cmd = Command::from("d20");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d100_die() {
        let expected = Command::new(1, D100, 0);
        let cmd = Command::from("d100");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_2d8() {
        let expected = Command::new(2, D8, 0);
        let cmd = Command::from("2d8");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_3d6_with_positive_modifier() {
        let expected = Command::new(3, D6, 2);
        let cmd = Command::from("3d6+2");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_1d20_with_negative_modifier() {
        let expected = Command::new(1, D20, -4);
        let cmd = Command::from("1d20-4");

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_executes_the_roll() {
        let result = Command::from("1d12+5").run();

        assert_eq!(result.die as u32, D12 as u32);
        assert_eq!(result.modifier, 5);
        assert_eq!(result.rolls.len(), 1);

        let sum: u32 = result.rolls.iter().sum();
        assert_eq!(result.total, sum as i32 + result.modifier);
    }
}
