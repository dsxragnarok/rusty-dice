/// ```
/// d4
/// 2d6
/// 4d10+1
/// 1d12-4
/// // split on 'd' | 'D'
/// ['', '4']
/// ['2', '6']
/// ['4', '10+1']
/// ['1', '12-4']
/// ```

use dice::Die;
use dice::Die::*;

pub struct Command {
    pub number_of_rolls: u32,
    pub die: Die,
    pub modifier: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dice::Die::*;

    #[test]
    fn it_parses_a_unit_d4_die() {
        let input_string = "d4";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D4 as u32);
    }

    #[test]
    fn it_parses_a_unit_d6_die() {
        let input_string = "d6";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D6 as u32);
    }

    #[test]
    fn it_parses_a_unit_d8_die() {
        let input_string = "d8";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D8 as u32);
    }

    #[test]
    fn it_parses_a_unit_d10_die() {
        let input_string = "d10";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D10 as u32);
    }

    #[test]
    fn it_parses_a_unit_d12_die() {
        let input_string = "d12";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D12 as u32);
    }

    #[test]
    fn it_parses_a_unit_d20_die() {
        let input_string = "d20";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D20 as u32);
    }

    #[test]
    fn it_parses_a_unit_d100_die() {
        let input_string = "d100";
        let cmd = parse(input_string);

        assert_eq!(cmd.number_of_rolls, 0);
        assert_eq!(cmd.modifier, 0);
        assert_eq!(cmd.die as u32, D100 as u32);
    }
}
