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

pub struct Command {
    pub number_of_rolls: u32,
    pub die: dice::Die,
    pub modifier: i32,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_unit_d4_die() {
        let input_string = "d4";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D4 as u32);
    }

    #[test]
    fn it_parses_a_unit_d6_die() {
        let input_string = "d6";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D6 as u32);
    }

    #[test]
    fn it_parses_a_unit_d8_die() {
        let input_string = "d8";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D8 as u32);
    }

    #[test]
    fn it_parses_a_unit_d10_die() {
        let input_string = "d10";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D10 as u32);
    }

    #[test]
    fn it_parses_a_unit_d12_die() {
        let input_string = "d12";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D12 as u32);
    }

    #[test]
    fn it_parses_a_unit_d20_die() {
        let input_string = "d20";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D20 as u32);
    }

    #[test]
    fn it_parses_a_unit_d100_die() {
        let input_string = "d100";
        let cmd = parse(input_string);

        expect_eq!(cmd.number_of_rolls, 0);
        expect_eq!(cmd.modifier, 0);
        expect_eq!(cmd.die as u32, dice::Die::D100 as u32);
    }
}
