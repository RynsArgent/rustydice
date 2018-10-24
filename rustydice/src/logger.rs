use chrono::{DateTime, Local, TimeZone};
use dice::RollResult;

pub fn build_log(result: &RollResult, date_time: &DateTime<Local>) -> String {
	let RollResult { die, rolls, modifier, total, .. } = result;

    let modifier = if *modifier > 0 {
        format!("+{}", *modifier)
    } else if *modifier == 0 {
        String::from("")
    } else {
        format!("{}", *modifier)
    };

    format!("[{}] You rolled {}d{}{} for {}\n >>> {:?}",
        date_time.format("%m/%d/%Y %H:%M:%S"),
        rolls.len(),
        *die as u32,
        modifier,
        total,
        rolls
    )
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
            number_of_rolls: 4,
            modifier: 4,
            rolls: vec![3,4,8,1],
            total: 20,
        };

        let log = build_log(&result, &Local::now());

        assert!(log.find("You rolled 4d8+4 for 20") != None);
    }

	#[test]
    fn it_constructs_the_log_text_from_roll_result_with_negative_modifier() {
        let result = RollResult {
            die: D4,
            number_of_rolls: 2,
            modifier: -3,
            rolls: vec![3,4],
            total: 4,
        };

        let log = build_log(&result, &Local::now());;

        assert!(log.find("You rolled 2d4-3 for 4") != None);
    }

	#[test]
    fn it_constructs_the_log_text_from_roll_result_with_zero_modifier() {
        let result = RollResult {
            die: D6,
            number_of_rolls: 3,
            modifier: 0,
            rolls: vec![6,4,6],
            total: 16,
        };

        let log = build_log(&result, &Local::now());

        assert!(log.find("You rolled 3d6 for 16") != None);
    }

	#[test]
    fn it_constructs_the_log_text_with_vector_of_individual_rolls() {
        let result = RollResult {
            die: D2,
            number_of_rolls: 12,
            modifier: 0,
            rolls: vec![2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1],
            total: 12,
        };

        let log = build_log(&result, &Local::now());

        assert!(log.find(">>> [2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1]") != None);
    }

	#[test]
    fn it_constructs_the_log_text_with_timestamp() {
        let result = RollResult {
            die: D8,
            number_of_rolls: 1,
            modifier: 0,
            rolls: vec![8],
            total: 8,
        };

        let expected_timestamp = "09/22/2018 01:12:06";
        let datetime = Local.datetime_from_str(expected_timestamp, "%m/%d/%Y %H:%M:%S").unwrap();
        let log = build_log(&result, &datetime);

        assert!(log.find(expected_timestamp) != None);
    }
}