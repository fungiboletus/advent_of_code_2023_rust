/*
    Part 1:
    We will do a Manhattan distance. The universe expand so
    each column and row will have a summed distance.
*/

use ndarray::Array2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many0, separated_list0},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Galaxy,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

fn empty_lists_indexes(bools: &[bool]) -> Vec<usize> {
    bools
        .iter()
        .scan((0, false), |state, &val| {
            let (sum, prev) = *state;
            *state = (sum + if prev { 2 } else { 1 }, val);
            Some(state.0)
        })
        .collect()
}

fn couples(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).flat_map(move |i| (i + 1..n).map(move |j| (i, j)))
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<Tile>> {
    let (left, data) = separated_list0(line_ending, many0(one_of(".#")))(data)?;

    let nb_rows = data.len();
    let nb_cols = data.first().map_or(0, |row| row.len());

    Ok((
        left,
        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| {
            Tile::from_char(data[row][col])
        }),
    ))
}

pub fn day_11_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");

    println!("{:?}", grid);

    // Iterate row by row on the grid Array2
    let empty_rows: Vec<bool> = grid
        .rows()
        .into_iter()
        .map(|row| row.iter().all(|tile| *tile == Tile::Empty))
        .collect();

    let empty_columns: Vec<bool> = grid
        .columns()
        .into_iter()
        .map(|column| column.iter().all(|tile| *tile == Tile::Empty))
        .collect();

    println!("{:?}", empty_rows);
    println!("{:?}", empty_columns);

    let index_rows: Vec<usize> = empty_lists_indexes(&empty_rows);
    let index_columns: Vec<usize> = empty_lists_indexes(&empty_columns);

    println!("{:?}", index_rows);
    println!("{:?}", index_columns);

    // find all the galaxies and but them into a list
    let list_of_galaxies: Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|(_, tile)| **tile == Tile::Galaxy)
        .map(|((row, col), _)| (row, col))
        .collect();

    println!("{:?}", list_of_galaxies);
    let nb_galaxies = list_of_galaxies.len();

    couples(nb_galaxies)
        .map(|(galaxy_a, galaxy_b)| {
            println!("{:?} {:?}", galaxy_a, galaxy_b);

            let (row_a, col_a) = list_of_galaxies[galaxy_a];
            let (row_b, col_b) = list_of_galaxies[galaxy_b];

            let corrected_row_a = index_rows[row_a];
            let corrected_row_b = index_rows[row_b];
            let corrected_col_a = index_columns[col_a];
            let corrected_col_b = index_columns[col_b];

            // manhattan distance
            (corrected_row_a as i64 - corrected_row_b as i64).abs()
                + (corrected_col_a as i64 - corrected_col_b as i64).abs()
        })
        .sum()
}

pub fn day_11_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(day_11_part_1(EXAMPLE), 374);
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(day_11_part_2(EXAMPLE), 42);
    }
}
