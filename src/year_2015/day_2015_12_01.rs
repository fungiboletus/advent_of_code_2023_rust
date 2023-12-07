/*
    2015 day 1 puzzle to have it easy.
*/

pub fn day_2015_12_01_part_1(data: &str) -> i64 {
    // iterate char by char
    data.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

pub fn day_2015_12_01_part_2(data: &str) -> i64 {
    // Without copilot since the part 1 went a bit too easily.
    // It feels like 2015 again.
    let mut floor = 0_i64;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return i as i64 + 1;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2015_12_01_part_1() {
        // Note copilot knows about the test cases
        assert_eq!(day_2015_12_01_part_1("(())"), 0);
        assert_eq!(day_2015_12_01_part_1("()()"), 0);
        assert_eq!(day_2015_12_01_part_1("((("), 3);
        assert_eq!(day_2015_12_01_part_1("(()(()("), 3);
        assert_eq!(day_2015_12_01_part_1("))((((("), 3);
        assert_eq!(day_2015_12_01_part_1("())"), -1);
        assert_eq!(day_2015_12_01_part_1("))("), -1);
        assert_eq!(day_2015_12_01_part_1(")))"), -3);
        assert_eq!(day_2015_12_01_part_1(")())())"), -3);
    }

    #[test]
    fn test_day_2015_12_01_part_2() {
        assert_eq!(day_2015_12_01_part_2(")"), 1);
        assert_eq!(day_2015_12_01_part_2("()())"), 5);
    }
}
