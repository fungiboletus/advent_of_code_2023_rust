/*
    A relatively easier day when you have a nice crate to work with matrices.
*/

use rayon::prelude::*;

use ndarray::{s, Array2, ArrayView2};

use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{count, many1, separated_list0, separated_list1},
    IResult,
};

fn parse_pattern(data: &str) -> IResult<&str, Array2<bool>> {
    map(
        separated_list1(line_ending, many1(map(one_of(".#"), |c| c == '#'))),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col])
        },
    )(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Array2<bool>>> {
    separated_list0(count(line_ending, 2), parse_pattern)(data)
}

#[allow(dead_code)]
fn print_pattern(row: &ArrayView2<bool>) {
    for i in 0..row.nrows() {
        for j in 0..row.ncols() {
            print!("{}", if row[[i, j]] { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

// we need to check whether we have mirrorred rows,
// so we iterate over the rows and try to find a match
fn compute_mirrored_rows(pattern: &ArrayView2<bool>) -> Option<usize> {
    let nb_rows = pattern.nrows();

    for split_row in 1..nb_rows {
        let nb_rows_to_compare = split_row.min(nb_rows - split_row);
        // n rows before split_row
        let rows_before = pattern.slice(s![split_row - nb_rows_to_compare..split_row, ..]);
        // n rows after split row, notice the ;-1 to reverse the rows order
        let rows_after_flipped =
            pattern.slice(s![split_row..split_row + nb_rows_to_compare;-1, ..]);
        if rows_after_flipped == rows_before {
            return Some(split_row);
        }
    }
    return None;
}

fn count_differing_elements(matrix1: &ArrayView2<bool>, matrix2: &ArrayView2<bool>) -> usize {
    if matrix1.shape() != matrix2.shape() {
        panic!("Matrices must be of the same shape");
    }

    matrix1
        .iter()
        .zip(matrix2.iter())
        .filter(|(&elem1, &elem2)| elem1 != elem2)
        .count()
}

// part 2 is not looking for equality but rows that have exactly one cell
// different.
fn compute_almost_mirrored_rows(pattern: &ArrayView2<bool>) -> Option<usize> {
    let nb_rows = pattern.nrows();

    for split_row in 1..nb_rows {
        let nb_rows_to_compare = split_row.min(nb_rows - split_row);
        let rows_before = pattern.slice(s![split_row - nb_rows_to_compare..split_row, ..]);
        let rows_after_flipped =
            pattern.slice(s![split_row..split_row + nb_rows_to_compare;-1, ..]);
        let nb_diffs = count_differing_elements(&rows_before, &rows_after_flipped);
        if nb_diffs == 1 {
            return Some(split_row);
        }
    }
    return None;
}
pub fn day_13_part_1(data: &str) -> i64 {
    let (_, patterns) = parse_input_data(data).expect("Failed to parse input data");

    patterns
        .par_iter()
        .map(|p| match compute_mirrored_rows(&p.view()) {
            Some(split_row) => split_row * 100,
            None => match compute_mirrored_rows(&p.t().view()) {
                Some(split_row) => split_row,
                None => panic!("No mirrors found"),
            },
        })
        .sum::<usize>() as i64
}

pub fn day_13_part_2(data: &str) -> i64 {
    let (_, patterns) = parse_input_data(data).expect("Failed to parse input data");

    patterns
        .par_iter()
        .map(|p| match compute_almost_mirrored_rows(&p.view()) {
            Some(split_row) => split_row * 100,
            None => match compute_almost_mirrored_rows(&p.t().view()) {
                Some(split_row) => split_row,
                None => panic!("No almost mirrors found"),
            },
        })
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_day_13_part_1() {
        assert_eq!(day_13_part_1(EXAMPLE), 405);
    }

    #[test]
    fn test_day_13_part_2() {
        assert_eq!(day_13_part_2(EXAMPLE), 400);
    }
}
