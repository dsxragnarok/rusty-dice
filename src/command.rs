use dice::Die;
use dice::Die::*;

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
}

pub fn parse(input_text: &str) -> Command {
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
        let input_string = "d4";
        let expected = Command::new(1, D4, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d6_die() {
        let input_string = "d6";
        let expected = Command::new(1, D6, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d8_die() {
        let input_string = "d8";
        let expected = Command::new(1, D8, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d10_die() {
        let input_string = "d10";
        let expected = Command::new(1, D10, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d12_die() {
        let input_string = "d12";
        let expected = Command::new(1, D12, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d20_die() {
        let input_string = "d20";
        let expected = Command::new(1, D20, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_a_unit_d100_die() {
        let input_string = "d100";
        let expected = Command::new(1, D100, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_2d8() {
        let input_string = "2d8";
        let expected = Command::new(2, D8, 0);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_3d6_with_positive_modifier() {
        let input_string = "3d6+2";
        let expected = Command::new(3, D6, 2);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }

    #[test]
    fn it_parses_1d20_with_negative_modifier() {
        let input_string = "1d20-4";
        let expected = Command::new(1, D20, -4);
        let cmd = parse(input_string);

        assert_command_is_valid(expected, cmd);
    }
}
