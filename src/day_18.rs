/*
    Plan for part 1:
        - parse the input
        - compute the 2d index of each tile in a vec, starting at (0,0)
        - build a 2d matrice of the tiles
        - detect the interior, somehow
        - flood fill

    Plan for part 2:
        - realise that it wasn't going to be about colours, it was a trap
        - check the subreddit to find a keyword about the right algorithm to use
        - the most straightforward solution is "Shoelace formula" with "Pick's theorem"
        - Apparently, day 10 could also be solved using this.
        - yolo it
*/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, satisfy, u64},
    combinator::{map, map_res, value},
    multi::{count, separated_list0},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
    colour: u32,
}

fn parse_6digits_hex(data: &str) -> IResult<&str, u32> {
    map_res(
        count(satisfy(|c: char| c.is_ascii_hexdigit()), 6),
        |s: Vec<char>| u32::from_str_radix(&s.into_iter().collect::<String>(), 16),
    )(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(
        line_ending,
        map(
            tuple((
                alt((
                    value(Direction::Up, tag("U")),
                    value(Direction::Down, tag("D")),
                    value(Direction::Left, tag("L")),
                    value(Direction::Right, tag("R")),
                )),
                tag(" "),
                u64,
                tag(" (#"),
                parse_6digits_hex,
                tag(")"),
            )),
            |(direction, _, steps, _, colour, _)| Instruction {
                direction,
                steps: steps as usize,
                colour,
            },
        ),
    )(data)
}

/*
fn build_matrice(instructions: &[Instruction]) -> Array2<bool> {
    let mut tiles = Vec::new();
    let mut current_tile = (0_isize, 0_isize);

    for instruction in instructions {
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => current_tile.0 -= 1,
                Direction::Down => current_tile.0 += 1,
                Direction::Left => current_tile.1 -= 1,
                Direction::Right => current_tile.1 += 1,
            }
            tiles.push(current_tile);
        }
    }

    // do everything in one go
    let (min_row, max_row, min_col, max_col) = tiles.iter().fold(
        (0_isize, 0_isize, 0_isize, 0_isize),
        |(min_row, max_row, min_col, max_col), (row, col)| {
            (
                min_row.min(*row),
                max_row.max(*row),
                min_col.min(*col),
                max_col.max(*col),
            )
        },
    );

    // add 1 row and 1 column on each side
    let nb_rows = (max_row - min_row + 3) as usize;
    let nb_cols = (max_col - min_col + 3) as usize;

    let mut matrice = Array2::<bool>::from_elem((nb_rows, nb_cols), false);

    for (row, col) in tiles {
        matrice[((row - min_row + 1) as usize, (col - min_col + 1) as usize)] = true;
    }

    matrice
}

fn flood_fill_inside(matrice: &Array2<bool>) -> Array2<bool> {
    let dim = matrice.dim();
    let (nrows, ncols) = dim;
    let mut stack = Vec::new();

    let mut visited = Array2::<bool>::from_elem(dim, true);

    // Start with top left corner that is always empty
    // as we added 1 row and 1 column on each side
    stack.push((0, 0));

    while let Some((row, col)) = stack.pop() {
        if visited[(row, col)] {
            visited[(row, col)] = false;

            if row > 0 && !matrice[(row - 1, col)] {
                stack.push((row - 1, col));
            }
            if row < nrows - 1 && !matrice[(row + 1, col)] {
                stack.push((row + 1, col));
            }
            if col > 0 && !matrice[(row, col - 1)] {
                stack.push((row, col - 1));
            }
            if col < ncols - 1 && !matrice[(row, col + 1)] {
                stack.push((row, col + 1));
            }
        }
    }

    visited
}

#[allow(dead_code)]
fn pretty_print_matrice(matrice: &Array2<bool>) {
    for row in matrice.rows() {
        for tile in row {
            print!("{}", if *tile { '#' } else { '.' });
        }
        println!();
    }
}

pub fn day_18_part_1(data: &str) -> i64 {
    let (_, instructions) = parse_input_data(data).expect("Failed to parse input data");
    let matrice = build_matrice(&instructions);
    let flooded = flood_fill_inside(&matrice);
    //pretty_print_matrice(&flooded);
    // count the number of true
    flooded.iter().filter(|tile| **tile).count() as i64
}*/

fn polygon_inner_area(points: Vec<(isize, isize)>) -> isize {
    let mut area = 0;
    let n = points.len();

    for i in 0..n - 1 {
        let (x1, y1) = points[i];
        //let (x2, y2) = if i == n - 1 { points[0] } else { points[i + 1] };
        let (x2, y2) = points[i + 1];

        area += x1 * y2;
        area -= y1 * x2;
    }

    (area.abs() / 2) as isize
}

fn digging_area(instructions: &[Instruction]) -> isize {
    let mut points = Vec::new();
    let mut current_tile = (0_isize, 0_isize);
    let mut boundary_length = 0_isize;

    for instruction in instructions {
        let steps = instruction.steps as isize;
        boundary_length += steps;
        match instruction.direction {
            Direction::Up => current_tile.0 -= steps,
            Direction::Down => current_tile.0 += steps,
            Direction::Left => current_tile.1 -= steps,
            Direction::Right => current_tile.1 += steps,
        }
        points.push(current_tile);
    }

    // Pick's theorem
    polygon_inner_area(points) + boundary_length / 2 + 1
}

pub fn day_18_part_1(data: &str) -> i64 {
    let (_, instructions) = parse_input_data(data).expect("Failed to parse input data");
    digging_area(&instructions) as i64
}

pub fn day_18_part_2(data: &str) -> i64 {
    let (_, instructions) = parse_input_data(data).expect("Failed to parse input data");

    // We need to translate the instructions from the colour number to a new list of instructions
    let fixed_instructions = instructions
        .iter()
        .map(|instruction| {
            let colour = instruction.colour;
            // extract the last hex digit from the number using a mask
            let last_digit = colour & 0xF;
            let new_direction = match last_digit {
                0x0 => Direction::Right,
                0x1 => Direction::Down,
                0x2 => Direction::Left,
                0x3 => Direction::Up,
                _ => panic!("Unexpected last digit: {}", last_digit),
            };
            // the new steps is the number without the last digit, using a shift
            let new_steps = colour >> 4;
            Instruction {
                direction: new_direction,
                steps: new_steps as usize,
                colour,
            }
        })
        .collect::<Vec<_>>();

    digging_area(&fixed_instructions) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_day_18_part_1() {
        assert_eq!(day_18_part_1(EXAMPLE), 62);
    }

    #[test]
    fn test_day_18_part_2() {
        assert_eq!(day_18_part_2(EXAMPLE), 952408144115);
    }
}
