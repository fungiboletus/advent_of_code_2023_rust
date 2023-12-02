fn parse_input_data(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|line| line.chars().collect()).collect();
}

fn two_digits_to_number(first_digit: char, last_digit: char) -> i64 {
    let number = format!("{}{}", first_digit, last_digit)
        .parse::<i64>()
        .expect("Unable to parse number");
    return number;
}

pub fn day_1_part_1(data: &str) -> i64 {
    let lines = parse_input_data(data);

    let numbers = lines.iter().map(|line| {
        let first_digit = line
            .iter()
            .find(|&c| c.is_digit(10))
            .expect("The dataset doesn't contain any numbers");
        let last_digit = line
            .iter()
            .rfind(|&c| c.is_digit(10))
            .expect("The dataset doesn't contain any numbers");

        return two_digits_to_number(*first_digit, *last_digit);
    });

    return numbers.sum();
}

fn find_first_substring(data: &Vec<char>, search: &Vec<char>) -> Option<usize> {
    let result = data
        .windows(search.len())
        .position(|window| window == search);
    return result;
}

fn find_last_substring(data: &Vec<char>, search: &Vec<char>) -> Option<usize> {
    let result = data
        .windows(search.len())
        .rposition(|window| window == search);
    return result;
}

pub fn day_1_part_2(data: &str) -> i64 {
    let lines = parse_input_data(data);

    let numbers = lines.iter().map(|line| {
        let (mut index_first_digit, first_digit) = line
            .iter()
            .enumerate()
            .find(|(_, c)| c.is_digit(10))
            .unwrap_or((usize::MAX, &'0'));

        let (mut index_last_digit, last_digit) = line
            .iter()
            .enumerate()
            .rfind(|(_, c)| c.is_digit(10))
            .unwrap_or((0, &'0'));

        let mut first_digit = *first_digit;
        let mut last_digit = *last_digit;

        let full_text_data: Vec<(Vec<char>, char)> = vec![
            (vec!['o', 'n', 'e'], '1'),
            (vec!['t', 'w', 'o'], '2'),
            (vec!['t', 'h', 'r', 'e', 'e'], '3'),
            (vec!['f', 'o', 'u', 'r'], '4'),
            (vec!['f', 'i', 'v', 'e'], '5'),
            (vec!['s', 'i', 'x'], '6'),
            (vec!['s', 'e', 'v', 'e', 'n'], '7'),
            (vec!['e', 'i', 'g', 'h', 't'], '8'),
            (vec!['n', 'i', 'n', 'e'], '9'),
        ];

        for (search, replacement) in full_text_data {
            let search_first = find_first_substring(line, &search);
            match search_first {
                Some(index) => {
                    if index < index_first_digit {
                        index_first_digit = index;
                        first_digit = replacement;
                    }
                }
                None => {}
            }

            let search_last = find_last_substring(line, &search);
            match search_last {
                Some(index) => {
                    if index > index_last_digit {
                        index_last_digit = index;
                        last_digit = replacement;
                    }
                }
                None => {}
            }
        }

        return two_digits_to_number(first_digit, last_digit);
    });

    return numbers.sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_PART_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_day_1_part_1() {
        assert_eq!(day_1_part_1(EXAMPLE_PART_1), 142);
    }

    #[test]
    fn test_day_1_part_2() {
        assert_eq!(day_1_part_2(EXAMPLE_PART_1), 142);
        assert_eq!(day_1_part_2(EXAMPLE_PART_2), 281);
    }
}
