use dice::RollResult;

/// Constructs the log text given the RollResult
///
/// Parameters
/// * `result` - The a dice roll result object
///
/// Returns the formatted log String
pub fn build_log(result: &RollResult) -> String {
    let RollResult { die, rolls, modifier, total } = result;

    let modifier = if *modifier > 0 {
        format!("+{}", *modifier)
    } else if *modifier == 0 {
        String::from("")
    } else {
        format!("{}", *modifier)
    };

    format!("You rolled {}d{}{} for {}", rolls.len(), *die as u32, modifier, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dice::Die::*;
    use dice::RollResult;

    #[test]
    fn it_constructs_the_log_text_from_roll_result_with_positive_modifier() {
        let result = RollResult {
            die: D8,
            modifier: 4,
            rolls: vec![3,4,8,1],
            total: 20,
        };

        let log = build_log(result);

        assert_eq!(log, "You rolled 4d8+4 for 20");
    }

    #[test]
    fn it_constructs_the_log_text_from_roll_result_with_negative_modifier() {
        let result = RollResult {
            die: D4,
            modifier: -3,
            rolls: vec![3,4],
            total: 4,
        };

        let log = build_log(result);

        assert_eq!(log, "You rolled 2d4-3 for 4");
    }

    #[test]
    fn it_constructs_the_log_text_from_roll_result_with_zero_modifier() {
        let result = RollResult {
            die: D6,
            modifier: 0,
            rolls: vec![6],
            total: 6,
        };

        let log = build_log(result);

        assert_eq!(log, "You rolled 1d6 for 6");
    }
}
