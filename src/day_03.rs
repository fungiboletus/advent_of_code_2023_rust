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

pub fn day_3_part_1(data: &str) -> i64 {
    let data = parse_input_data(data);
    //println!("{:?}", data);
    let dim = data.dim();
    let (rows, cols) = dim;

    let adjacent_symbols_matrix = compute_nb_of_adjacent_symbols(&data);
    //println!("{:?}", adjacent_symbols_matrix);

    let mut sum: i64 = 0;

    // for each line
    for i in 0..rows {
        let mut start_text_j: isize = -1;
        let mut has_seen_symbol = false;
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
                    //println!("Found text: {:?}", number);
                    sum += number;
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
            sum += number;
        }
    }

    return sum;
}

pub fn day_3_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const EXAMPLE_PART_1B: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const EXAMPLE_PART_2: &str = "467..114..
...*......
..35...633
.......#..
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_day_3_part_1() {
        assert_eq!(day_3_part_1(EXAMPLE_PART_1), 4361);
        assert_eq!(day_3_part_1(EXAMPLE_PART_1B), 4361);
    }

    #[test]
    fn test_day_3_part_2() {
        assert_eq!(day_3_part_2(EXAMPLE_PART_1), 42);
    }
}
