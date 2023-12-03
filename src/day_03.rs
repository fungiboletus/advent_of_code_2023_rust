use std::collections::HashSet;

use ndarray::Array2;

fn parse_input_data(data: &str) -> Array2<char> {
    let numbers = data
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<char>>();

    // compute sqrt of the number of elements, as it's a square
    let sqrt = (numbers.len() as f64).sqrt() as usize;

    return Array2::from_shape_vec((sqrt, sqrt), numbers).expect("Failed to parse input data");
}

#[inline]
fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}

fn compute_nb_of_adjacent_symbols(data: &Array2<char>) -> Array2<usize> {
    let dim = data.dim();
    let (rows, cols) = dim;
    // Create a zero matrix of the same size as the input data
    let mut result = Array2::<usize>::zeros(dim);

    for i in 0..rows {
        for j in 0..cols {
            let mut sum = 0;
            for di in 0.max(i as isize - 1) as usize..=(rows - 1).min(i + 1) {
                for dj in 0.max(j as isize - 1) as usize..=(cols - 1).min(j + 1) {
                    if is_symbol(data[(di, dj)]) {
                        sum += 1
                    }
                }
            }
            result[(i, j)] = sum;
        }
    }

    return result;
}

#[inline]
fn compute_number_from_matrix(
    data: &Array2<char>,
    row: usize,
    col_start: usize,
    length: usize,
) -> i64 {
    data.slice(ndarray::s![row, col_start..col_start + length])
        .iter()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .fold(0, |acc, x| acc * 10 + x)
}

#[inline]
fn fill_matrix_with_number(
    matrix: &mut Array2<i64>,
    row: usize,
    col_start: usize,
    length: usize,
    number: i64,
) {
    for i in col_start..col_start + length {
        matrix[(row, i)] = number;
    }
}

fn identify_part_numbers(data: &Array2<char>) -> (Vec<i64>, Array2<i64>) {
    let dim = data.dim();
    let (rows, cols) = dim;

    let adjacent_symbols_matrix = compute_nb_of_adjacent_symbols(&data);
    //println!("{:?}", adjacent_symbols_matrix);

    let mut part_numbers = Vec::<i64>::new();

    let mut part_numbers_matrix = Array2::<i64>::zeros(dim);

    // for each row
    for i in 0..rows {
        let mut start_text_j: isize = -1;
        let mut has_seen_symbol = false;
        // for each column
        for j in 0..cols {
            let c = data[(i, j)];
            let is_digit = c.is_digit(10);
            if is_digit {
                if start_text_j == -1 {
                    start_text_j = j as isize;
                }
                if !has_seen_symbol {
                    let nb_adjacent_symbols = adjacent_symbols_matrix[(i, j)];
                    if nb_adjacent_symbols > 0 {
                        has_seen_symbol = true;
                    }
                }
            } else {
                if start_text_j != -1 && has_seen_symbol {
                    let end_text_j = j as isize - 1;
                    let text_len = end_text_j - start_text_j + 1;
                    // let number = &data
                    //     .slice(ndarray::s![i, start_text_j as usize..=end_text_j as usize]);
                    let number = compute_number_from_matrix(
                        &data,
                        i,
                        start_text_j as usize,
                        text_len as usize,
                    );
                    fill_matrix_with_number(
                        &mut part_numbers_matrix,
                        i,
                        start_text_j as usize,
                        text_len as usize,
                        number,
                    );
                    //println!("Found text: {:?}", number);
                    part_numbers.push(number);
                }
                start_text_j = -1;
                has_seen_symbol = false;
            }
        }
        if start_text_j != -1 && has_seen_symbol {
            let end_text_j = cols as isize - 1;
            let text_len = end_text_j - start_text_j + 1;
            let number =
                compute_number_from_matrix(&data, i, start_text_j as usize, text_len as usize);
            //println!("Found text: {:?}", number);
            fill_matrix_with_number(
                &mut part_numbers_matrix,
                i,
                start_text_j as usize,
                text_len as usize,
                number,
            );
            part_numbers.push(number);
        }
    }

    return (part_numbers, part_numbers_matrix);
}

pub fn day_3_part_1(data: &str) -> i64 {
    let data = parse_input_data(data);
    let (part_numbers, _) = identify_part_numbers(&data);
    return part_numbers.iter().sum();
}

pub fn day_3_part_2(data: &str) -> i64 {
    let data = parse_input_data(data);
    let (_, part_numbers_matrix) = identify_part_numbers(&data);

    let mut sum: i64 = 0;

    for ((row, col), symbol) in data.indexed_iter() {
        if *symbol == '*' {
            let mut adjacent_part_numbers = HashSet::<i64>::new();
            for di in 0.max(row as isize - 1) as usize..=(data.dim().0 - 1).min(row + 1) {
                for dj in 0.max(col as isize - 1) as usize..=(data.dim().1 - 1).min(col + 1) {
                    let adjacent_part_number = part_numbers_matrix[(di, dj)];
                    if adjacent_part_number > 0 {
                        adjacent_part_numbers.insert(adjacent_part_number);
                    }
                }
            }

            if adjacent_part_numbers.len() == 2 {
                sum += adjacent_part_numbers.iter().product::<i64>();
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const EXAMPLE_B: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_day_3_part_1() {
        assert_eq!(day_3_part_1(EXAMPLE), 4361);
        assert_eq!(day_3_part_1(EXAMPLE_B), 4361);
        // 528799
    }

    #[test]
    fn test_day_3_part_2() {
        assert_eq!(day_3_part_2(EXAMPLE), 467835);
    }
}
