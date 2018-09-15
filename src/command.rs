use dice::Die;
use dice::Die::*;

pub struct Command {
    pub number_of_rolls: u32,
    pub die: Die,
    pub modifier: i32,
}

pub fn parse(input_text: &str) -> Command {
    let string_parts: Vec<&str> = input_text.split('d').collect();

    let n = string_parts[0];
    let d = string_parts[1];

    let die = match d {
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

    Command {
        die,
        modifier: 0,
        number_of_rolls: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dice::Die::*;

    #[test]
    fn it_parses_a_unit_d4_die() {
        let input_string = "d4";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D4 as u32);
    }

    #[test]
    fn it_parses_a_unit_d6_die() {
        let input_string = "d6";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D6 as u32);
    }

    #[test]
    fn it_parses_a_unit_d8_die() {
        let input_string = "d8";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D8 as u32);
    }

    #[test]
    fn it_parses_a_unit_d10_die() {
        let input_string = "d10";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D10 as u32);
    }

    #[test]
    fn it_parses_a_unit_d12_die() {
        let input_string = "d12";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D12 as u32);
    }

    #[test]
    fn it_parses_a_unit_d20_die() {
        let input_string = "d20";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D20 as u32);
    }

    #[test]
    fn it_parses_a_unit_d100_die() {
        let input_string = "d100";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 1);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D100 as u32);
    }
}
