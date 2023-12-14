/*
    Part 1: it sounds like we need to work column by column
    and create slices on the areas between the unmovable cube shaped rows.
    then we can sort those areas, with rounded rocks first and empty then.
    then, we need to count based on the row number.
*/

use std::collections::HashMap;

use ndarray::{s, Array2, ArrayView1, ArrayView2};

use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Empty,
    RoundedRock,
    CubeShapedRock,
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<Tile>> {
    map(
        separated_list1(
            line_ending,
            many1(map(one_of(".#O"), |c| match c {
                '.' => Tile::Empty,
                'O' => Tile::RoundedRock,
                '#' => Tile::CubeShapedRock,
                _ => unreachable!("Unknown tile"),
            })),
        ),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col])
        },
    )(data)
}

#[allow(dead_code)]
fn print_grid(grid: &ArrayView2<Tile>) {
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            print!(
                "{}",
                match grid[[i, j]] {
                    Tile::Empty => '.',
                    Tile::RoundedRock => 'O',
                    Tile::CubeShapedRock => '#',
                }
            );
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_column(column: &ArrayView1<Tile>) {
    column.iter().for_each(|&tile| {
        let symbol = match tile {
            Tile::Empty => '.',
            Tile::RoundedRock => 'O',
            Tile::CubeShapedRock => '#',
        };
        println!("{}", symbol);
    });
}

fn extract_subsections(column: &ArrayView1<Tile>) -> Vec<(usize, usize)> {
    let mut subsections = Vec::new();
    let mut previous_row = 0;

    for (row, tile) in column.iter().enumerate() {
        if let Tile::CubeShapedRock = tile {
            if previous_row != row {
                subsections.push((previous_row, row));
            }
            previous_row = row + 1;
        }
    }

    // Handle the last subsection if necessary
    if previous_row < column.len() {
        subsections.push((previous_row, column.len()));
    }

    subsections
}

enum TiltDirection {
    North,
    West,
    South,
    East,
}

fn tilt_vertically(mut grid: Array2<Tile>, direction: TiltDirection) -> Array2<Tile> {
    let rounded_rock_first: std::cmp::Ordering;
    let rounded_rock_second: std::cmp::Ordering;
    match direction {
        TiltDirection::North => {
            rounded_rock_first = std::cmp::Ordering::Less;
            rounded_rock_second = std::cmp::Ordering::Greater;
        }
        TiltDirection::South => {
            rounded_rock_first = std::cmp::Ordering::Greater;
            rounded_rock_second = std::cmp::Ordering::Less;
        }
        _ => {
            panic!("Don't use this function for horizontal tilting");
        }
    }
    // work column by column
    for mut column in grid.columns_mut().into_iter() {
        // We identify the subsections that we need to work on.
        // the subsections are the areas separated by cube shaped rocks.
        let subsections = extract_subsections(&column.view());

        for (start, end) in subsections {
            // We extract the subsection as a vector because we need to sort it
            // and ndarray::ArrayViewMut doesn't implement sort.
            let mut slice = column.slice(s![start..end]).to_vec();

            // We sort the subsection by putting the rounded rocks first.
            slice.sort_unstable_by(|a, b| match (a, b) {
                (Tile::CubeShapedRock, _) | (_, Tile::CubeShapedRock) => {
                    unreachable!("Cube shaped rock in the middle of a slice")
                }
                (Tile::RoundedRock, _) => rounded_rock_first,
                (_, Tile::RoundedRock) => rounded_rock_second,
                _ => std::cmp::Ordering::Equal,
            });

            // We update the column with the sorted subsection.
            for (row, tile) in slice.into_iter().enumerate() {
                column[start + row] = tile;
            }
        }
    }

    grid
}

fn tilt_horizontally(mut grid: Array2<Tile>, direction: TiltDirection) -> Array2<Tile> {
    let rounded_rock_first: std::cmp::Ordering;
    let rounded_rock_second: std::cmp::Ordering;
    match direction {
        TiltDirection::West => {
            rounded_rock_first = std::cmp::Ordering::Less;
            rounded_rock_second = std::cmp::Ordering::Greater;
        }
        TiltDirection::East => {
            rounded_rock_first = std::cmp::Ordering::Greater;
            rounded_rock_second = std::cmp::Ordering::Less;
        }
        _ => {
            panic!("Don't use this function for vertical tilting");
        }
    }
    // work row by row
    for mut row in grid.rows_mut().into_iter() {
        // We identify the subsections that we need to work on.
        // the subsections are the areas separated by cube shaped rocks.
        let subsections = extract_subsections(&row.view());

        for (start, end) in subsections {
            // We extract the subsection as a vector because we need to sort it
            // and ndarray::ArrayViewMut doesn't implement sort.
            let mut slice = row.slice(s![start..end]).to_vec();

            // We sort the subsection by putting the rounded rocks first.
            slice.sort_unstable_by(|a, b| match (a, b) {
                (Tile::CubeShapedRock, _) | (_, Tile::CubeShapedRock) => {
                    unreachable!("Cube shaped rock in the middle of a slice")
                }
                (Tile::RoundedRock, _) => rounded_rock_first,
                (_, Tile::RoundedRock) => rounded_rock_second,
                _ => std::cmp::Ordering::Equal,
            });

            // We update the column with the sorted subsection.
            for (column, tile) in slice.into_iter().enumerate() {
                row[start + column] = tile;
            }
        }
    }

    grid
}

pub fn day_14_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let nb_rows = grid.nrows();
    //print_grid(&grid.view());
    // We will mutate the grid so we create a copy
    tilt_vertically(grid.to_owned(), TiltDirection::North)
        .indexed_iter()
        // look at the rounded rocks
        .filter(|(_, tile)| **tile == Tile::RoundedRock)
        // count the number of rows below the rounded rock
        .map(|((row, _), _)| nb_rows - row)
        .sum::<usize>() as i64
}

// Part 2: bruteforce is not an option.
// I looked at a tip, which seems obvious in hindsight:
// the grid will cycle.
// So we can just compute the grid after 1 cycle, 2 cycles, 3 cycles, etc.
// until we find a cycle.
// Then we can compute the grid after 1_000_000_000 cycles.

fn cycle(mut grid: Array2<Tile>) -> Array2<Tile> {
    grid = tilt_vertically(grid, TiltDirection::North);
    grid = tilt_horizontally(grid, TiltDirection::West);
    grid = tilt_vertically(grid, TiltDirection::South);
    grid = tilt_horizontally(grid, TiltDirection::East);
    return grid;
}

pub fn day_14_part_2(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let nb_rows = grid.nrows();
    let mut work_grid = grid.to_owned();

    // We will cache the grids we have already seen
    // Key is the grid, value is the index of the cycle
    let mut previous_grids: HashMap<Array2<Tile>, usize> = HashMap::new();
    previous_grids.insert(work_grid.clone(), 0);

    let nb_cycles = 1_000_000_000_usize;
    // hopefully the cycle is reached well before 1_000_000_000
    let mut cycle_start = 0_usize;
    let mut cycle_length = nb_cycles;
    for i in 1..=nb_cycles {
        work_grid = cycle(work_grid);
        if let Some(previous_i) = previous_grids.insert(work_grid.clone(), i) {
            cycle_start = previous_i;
            cycle_length = i - previous_i;
            break;
        }
    }

    // Index of the grid after 1_000_000_000 cycles
    let cycle_index_stop = cycle_start + (nb_cycles - cycle_start) % cycle_length;

    // find in the previous grid the key corresponding to the cycle_index_stop value
    let (final_grid, _) = previous_grids
        .iter()
        .find(|(_, &value)| value == cycle_index_stop)
        .expect("Cycle index not found");

    final_grid
        .indexed_iter()
        .filter(|(_, tile)| **tile == Tile::RoundedRock)
        .map(|((row, _), _)| nb_rows - row)
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_day_14_part_1() {
        assert_eq!(day_14_part_1(EXAMPLE), 136);
    }

    #[test]
    fn test_day_14_part_2() {
        assert_eq!(day_14_part_2(EXAMPLE), 64);
    }
}
