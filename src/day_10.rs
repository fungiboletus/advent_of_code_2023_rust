/*
    Part 1 sounds like we need a breadth first search.

    The code is a bit verbose because I don't like to work on abstract data structures.
*/

use std::collections::VecDeque;

use ndarray::Array2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many0, separated_list0},
    IResult,
};

// I prefer to work on an enum than the ascii characters.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    LNorthToEast,
    JNorthToWest,
    SevenSouthToWest,
    FSouthToEast,
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::LNorthToEast,
            'J' => Tile::JNorthToWest,
            '7' => Tile::SevenSouthToWest,
            'F' => Tile::FSouthToEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<Tile>> {
    let (left, data) = separated_list0(line_ending, many0(one_of("|-LJ7F.S")))(data)?;

    let nb_rows = data.len();
    let nb_cols = data.first().map_or(0, |row| row.len());

    Ok((
        left,
        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| {
            Tile::from_char(data[row][col])
        }),
    ))
}

fn visit_main_pipe(grid: &Array2<Tile>) -> (u64, Array2<u64>, (usize, usize), Tile) {
    // The problem could be solve with a recursive function
    // But I will manage the stack manually to make sure it is optimised,
    // as I don't want to rely on the compiler to optimise it.

    // (row, col, distance)
    let mut stack: VecDeque<(usize, usize, u64)> = VecDeque::new();

    // Need to find the starting point
    let start = grid
        .indexed_iter()
        .find(|(_, ref tile)| **tile == Tile::Start)
        .expect("Failed to find the starting point")
        .0;

    let (nb_rows, nb_cols) = grid.dim();

    let mut has_left = false;
    let mut has_right = false;
    let mut has_up = false;
    let mut has_down = false;

    // look around the starting point and add the tiles connected to it
    if start.1 > 0 {
        let left_tile = grid[(start.0, start.1 - 1)];
        if left_tile == Tile::Horizontal
            || left_tile == Tile::LNorthToEast
            || left_tile == Tile::FSouthToEast
        {
            has_left = true;
            stack.push_back((start.0, start.1 - 1, 1));
        }
    }

    if start.1 < nb_cols - 1 {
        let right_tile = grid[(start.0, start.1 + 1)];
        if right_tile == Tile::Horizontal
            || right_tile == Tile::JNorthToWest
            || right_tile == Tile::SevenSouthToWest
        {
            has_right = true;
            stack.push_back((start.0, start.1 + 1, 1));
        }
    }
    if start.0 > 0 {
        let up_tile = grid[(start.0 - 1, start.1)];
        if up_tile == Tile::Vertical
            || up_tile == Tile::SevenSouthToWest
            || up_tile == Tile::FSouthToEast
        {
            has_up = true;
            stack.push_back((start.0 - 1, start.1, 1));
        }
    }
    if start.0 < nb_rows - 1 {
        let down_tile = grid[(start.0 + 1, start.1)];
        if down_tile == Tile::Vertical
            || down_tile == Tile::LNorthToEast
            || down_tile == Tile::JNorthToWest
        {
            has_down = true;
            stack.push_back((start.0 + 1, start.1, 1));
        }
    }

    // compute the type of the starting point
    // (useful for part 2)
    let start_tile = if has_left && has_right {
        Tile::Horizontal
    } else if has_up && has_down {
        Tile::Vertical
    } else if has_left && has_down {
        Tile::SevenSouthToWest
    } else if has_left && has_up {
        Tile::JNorthToWest
    } else if has_right && has_up {
        Tile::LNorthToEast
    } else if has_right && has_down {
        Tile::FSouthToEast
    } else {
        panic!("Invalid starting point: too many or too few connections");
    };

    // created a matrix of visited tiles, could be a bitfield, but
    // it may be nicer to know the distance instead
    let mut visited = Array2::<u64>::zeros((nb_rows, nb_cols));

    // put the starting point as a visited tile
    visited[start] = u64::MAX;

    let mut highest_distance: u64 = 0;

    while let Some((row, col, distance)) = stack.pop_front() {
        if row >= nb_rows || col >= nb_cols {
            unreachable!("We should never be out of bound");
        }

        let visited_distance = visited[(row, col)];
        if visited_distance != 0 {
            continue;
        }

        visited[(row, col)] = distance;

        let tile = &grid[(row, col)];

        let side_a: (usize, usize);
        let side_b: (usize, usize);

        match tile {
            Tile::Start => {
                continue;
            }
            Tile::Ground => {
                continue;
            }
            Tile::Vertical => {
                side_a = (row + 1, col);
                side_b = (row - 1, col);
            }
            Tile::Horizontal => {
                side_a = (row, col + 1);
                side_b = (row, col - 1);
            }
            Tile::LNorthToEast => {
                side_a = (row - 1, col);
                side_b = (row, col + 1);
            }
            Tile::JNorthToWest => {
                side_a = (row - 1, col);
                side_b = (row, col - 1);
            }
            Tile::SevenSouthToWest => {
                side_a = (row + 1, col);
                side_b = (row, col - 1);
            }
            Tile::FSouthToEast => {
                side_a = (row + 1, col);
                side_b = (row, col + 1);
            }
        }

        if distance > highest_distance {
            highest_distance = distance;
        }

        if side_a.0 < nb_rows && side_a.1 < nb_cols {
            stack.push_back((side_a.0, side_a.1, distance + 1));
        }
        if side_b.0 < nb_rows && side_b.1 < nb_cols {
            stack.push_back((side_b.0, side_b.1, distance + 1));
        }
    }

    (highest_distance, visited, start, start_tile)
}

pub fn day_10_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let (highest_distance, _, _, _) = visit_main_pipe(&grid);
    highest_distance as i64
}

#[allow(dead_code)]
fn pretty_print_input(input: &str) {
    let mut output = String::new();

    for c in input.chars() {
        match c {
            'F' => output.push('\u{250F}'),
            'J' => output.push('\u{251B}'),
            'L' => output.push('\u{2517}'),
            '7' => output.push('\u{2513}'),
            '|' => output.push('\u{2503}'),
            '-' => output.push('\u{2501}'),
            _ => output.push(c),
        };
    }

    println!("{}", output);
}

#[allow(dead_code)]
fn pretty_print_grid(grid: &Array2<Tile>) {
    // print row by row
    for row in grid.outer_iter() {
        for tile in row {
            match tile {
                Tile::Vertical => print!("\u{2503}"),
                Tile::Horizontal => print!("\u{2501}"),
                Tile::LNorthToEast => print!("\u{2517}"),
                Tile::JNorthToWest => print!("\u{251B}"),
                Tile::SevenSouthToWest => print!("\u{2513}"),
                Tile::FSouthToEast => print!("\u{250F}"),
                Tile::Ground => print!("."),
                Tile::Start => print!("S"),
            }
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum VisitedState {
    None,
    NotVisitedGround,
    NotVisitedPipe,
    FromTopLeft,
    FromTopRight,
    FromBottomLeft,
    FromBottomRight,
}

#[allow(dead_code)]
fn pretty_print_visited(grid: &Array2<VisitedState>, grid2: &Array2<Tile>) {
    for (row_index, row) in grid.outer_iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            match tile {
                VisitedState::None
                | VisitedState::NotVisitedGround
                | VisitedState::NotVisitedPipe => {
                    let tile2 = grid2[(row_index, col_index)];
                    match tile2 {
                        Tile::Vertical => print!("\u{2503}"),
                        Tile::Horizontal => print!("\u{2501}"),
                        Tile::LNorthToEast => print!("\u{2517}"),
                        Tile::JNorthToWest => print!("\u{251B}"),
                        Tile::SevenSouthToWest => print!("\u{2513}"),
                        Tile::FSouthToEast => print!("\u{250F}"),
                        Tile::Ground => print!("."),
                        Tile::Start => print!("S"),
                    }
                }
                VisitedState::FromTopLeft => print!("a"),
                VisitedState::FromTopRight => print!("b"),
                VisitedState::FromBottomLeft => print!("c"),
                VisitedState::FromBottomRight => print!("d"),
            }
        }
        println!();
    }
}

/*
    Part 2 is a bit more complicated because of the rules,
    as the filling algorithm needs relatively quite some code.
*/

pub fn day_10_part_2(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let (_, visited_by_main_pipe, start_main_pipe, start_type_main_pipe) = visit_main_pipe(&grid);

    let (nb_rows, nb_cols) = grid.dim();

    let mut visited = Array2::<VisitedState>::from_elem((nb_rows, nb_cols), VisitedState::None);

    // we clean up all the pipes that are not visited, they are to be ignored
    // so they are replaced by ground for simplicity
    let mut grid = grid;
    for ((row, col), tile) in grid.indexed_iter_mut() {
        if visited_by_main_pipe[(row, col)] == 0 {
            *tile = Tile::Ground;
            visited[(row, col)] = VisitedState::NotVisitedGround;
        } else {
            visited[(row, col)] = VisitedState::NotVisitedPipe;
        }
    }

    // The start tile is also replaced by the only shape that it could be
    // so it's a tight loop
    grid[(start_main_pipe.0, start_main_pipe.1)] = start_type_main_pipe;

    // coordinates of the tile to be visited, and state of the previous tile
    // so we know from which side of the pipes we are coming from
    let mut stack: Vec<(usize, usize, VisitedState)> = Vec::new();

    // allways start with the top left corner
    stack.push((0, 0, VisitedState::FromTopLeft));

    while let Some((row, col, previous_tile_state)) = stack.pop() {
        if row >= nb_rows || col >= nb_cols {
            unreachable!("We should never be out of bound");
        }

        let visited_tile_state = visited[(row, col)];
        if visited_tile_state != VisitedState::NotVisitedPipe
            && visited_tile_state != VisitedState::NotVisitedGround
        {
            continue;
        }
        match previous_tile_state {
            VisitedState::None | VisitedState::NotVisitedGround | VisitedState::NotVisitedPipe => {
                unreachable!("Invalid previous tile state");
            }
            visited_tile_state => {
                visited[(row, col)] = visited_tile_state;
            }
        }
        //pretty_print_visited(&visited, &grid);
        //println!("");

        let mut push_left = VisitedState::None;
        let mut push_right = VisitedState::None;
        let mut push_up = VisitedState::None;
        let mut push_down = VisitedState::None;

        let tile = &grid[(row, col)];
        match tile {
            Tile::Start => {
                unreachable!("We should never encounter the starting point");
            }
            Tile::Ground => {
                push_left = VisitedState::FromTopRight;
                push_right = VisitedState::FromTopLeft;
                push_up = VisitedState::FromBottomLeft;
                push_down = VisitedState::FromTopLeft;
            }
            Tile::Vertical => match previous_tile_state {
                VisitedState::FromTopLeft | VisitedState::FromBottomLeft => {
                    push_up = VisitedState::FromBottomLeft;
                    push_down = VisitedState::FromTopLeft;
                    push_left = VisitedState::FromTopRight;
                }
                VisitedState::FromTopRight | VisitedState::FromBottomRight => {
                    push_up = VisitedState::FromBottomRight;
                    push_down = VisitedState::FromTopRight;
                    push_right = VisitedState::FromTopLeft;
                }
                _ => {}
            },
            Tile::Horizontal => match previous_tile_state {
                VisitedState::FromTopLeft | VisitedState::FromTopRight => {
                    push_left = VisitedState::FromTopRight;
                    push_right = VisitedState::FromTopLeft;
                    push_up = VisitedState::FromBottomLeft;
                }
                VisitedState::FromBottomLeft | VisitedState::FromBottomRight => {
                    push_left = VisitedState::FromBottomRight;
                    push_right = VisitedState::FromBottomLeft;
                    push_down = VisitedState::FromTopLeft;
                }
                _ => {}
            },
            Tile::LNorthToEast => match previous_tile_state {
                VisitedState::FromTopLeft
                | VisitedState::FromBottomLeft
                | VisitedState::FromBottomRight => {
                    push_left = VisitedState::FromTopRight;
                    push_right = VisitedState::FromBottomLeft;
                    push_up = VisitedState::FromBottomLeft;
                    push_down = VisitedState::FromTopLeft;
                }
                VisitedState::FromTopRight => {
                    push_up = VisitedState::FromBottomRight;
                    push_right = VisitedState::FromTopLeft;
                }
                _ => {}
            },
            Tile::JNorthToWest => match previous_tile_state {
                VisitedState::FromBottomRight
                | VisitedState::FromBottomLeft
                | VisitedState::FromTopRight => {
                    push_left = VisitedState::FromBottomRight;
                    push_right = VisitedState::FromTopLeft;
                    push_up = VisitedState::FromBottomRight;
                    push_down = VisitedState::FromTopLeft;
                }
                VisitedState::FromTopLeft => {
                    push_up = VisitedState::FromBottomLeft;
                    push_left = VisitedState::FromTopRight;
                }
                _ => {}
            },
            Tile::SevenSouthToWest => match previous_tile_state {
                VisitedState::FromTopLeft
                | VisitedState::FromTopRight
                | VisitedState::FromBottomRight => {
                    push_left = VisitedState::FromTopRight;
                    push_right = VisitedState::FromTopLeft;
                    push_up = VisitedState::FromBottomLeft;
                    push_down = VisitedState::FromTopRight;
                }
                VisitedState::FromBottomLeft => {
                    push_down = VisitedState::FromTopLeft;
                    push_left = VisitedState::FromBottomRight;
                }
                _ => {}
            },
            Tile::FSouthToEast => match previous_tile_state {
                VisitedState::FromTopLeft
                | VisitedState::FromTopRight
                | VisitedState::FromBottomLeft => {
                    push_left = VisitedState::FromTopRight;
                    push_right = VisitedState::FromTopLeft;
                    push_up = VisitedState::FromBottomLeft;
                    push_down = VisitedState::FromTopLeft;
                }
                VisitedState::FromBottomRight => {
                    push_down = VisitedState::FromTopRight;
                    push_right = VisitedState::FromBottomLeft;
                }
                _ => {}
            },
        }
        if push_left != VisitedState::None && col > 0 {
            stack.push((row, col - 1, push_left));
        }
        if push_right != VisitedState::None && col < nb_cols - 1 {
            stack.push((row, col + 1, push_right));
        }
        if push_up != VisitedState::None && row > 0 {
            stack.push((row - 1, col, push_up));
        }
        if push_down != VisitedState::None && row < nb_rows - 1 {
            stack.push((row + 1, col, push_down));
        }
    }

    // pretty_print_grid(&grid);

    // count the number of not visited tiles
    visited.iter().fold(0, |acc, tile| {
        if *tile == VisitedState::NotVisitedGround {
            acc + 1
        } else {
            acc
        }
    }) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    /*
    .....
    .S━┓.
    .┃.┃.
    .┗━┛.
    .....
    */

    const EXAMPLE_B: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    /*
    ..┏┓.
    .┏┛┃.
    S┛.┗┓
    ┃┏━━┛
    ┗┛...
    */

    const EXAMPLE_C: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    /*
    ...........
    .S━━━━━━━┓.
    .┃┏━━━━━┓┃.
    .┃┃.....┃┃.
    .┃┃.....┃┃.
    .┃┗━┓.┏━┛┃.
    .┃..┃.┃..┃.
    .┗━━┛.┗━━┛.
    ...........
    */

    const EXAMPLE_D: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    /*
    ..........
    .S━━━━━━┓.
    .┃┏━━━━┓┃.
    .┃┃....┃┃.
    .┃┃....┃┃.
    .┃┗━┓┏━┛┃.
    .┃..┃┃..┃.
    .┗━━┛┗━━┛.
    ..........
    */

    const EXAMPLE_E: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    /*
    .┏━━━━┓┏┓┏┓┏┓┏━┓....
    .┃┏━━┓┃┃┃┃┃┃┃┃┏┛....
    .┃┃.┏┛┃┃┃┃┃┃┃┃┗┓....
    ┏┛┗┓┗┓┗┛┗┛┃┃┗┛.┗━┓..
    ┗━━┛.┗┓...┗┛S┓┏━┓┗┓.
    ....┏━┛..┏┓┏┛┃┗┓┗┓┗┓
    ....┗┓.┏┓┃┃┗┓┃.┗┓┗┓┃
    .....┃┏┛┗┛┃┏┛┃┏┓┃.┗┛
    ....┏┛┗━┓.┃┃.┃┃┃┃...
    ....┗━━━┛.┗┛.┗┛┗┛...
    */

    const EXAMPLE_F: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    /*
    ┏┏┓┏S┏┓┏┓┏┓┏┓┏┓┏━━━┓
    ┗┃┗┛┃┃┃┃┃┃┃┃┃┃┃┃┏━━┛
    ┏┗━┓┗┛┗┛┃┃┃┃┃┃┗┛┗━┓┓
    ┏━━┛┏━━┓┃┃┗┛┗┛┓┏┓┏┛━
    ┗━━━┛┏━┛┗┛.┃┃━┏┛┗┛┛┓
    ┃┏┃┏━┛┏━━━┓┏┓━┗┓┗┃┓┃
    ┃┏┏┛┏┓┗┓┏━┛┏┓┃┛┗━━━┓
    ┓━┗━┛┗┓┃┃┏┓┃┗┓┏━┓┏┓┃
    ┗.┗┓┗┏┛┃┃┃┃┃┏┛┗┓┃┃┗┛
    ┗┓┛┗┛┗━┛┗┛┗┛┗━━┛┗┛.┗
    */

    #[test]
    fn test_day_10_part_1() {
        assert_eq!(day_10_part_1(EXAMPLE_A), 4);
        assert_eq!(day_10_part_1(EXAMPLE_B), 8);
    }

    #[test]
    fn test_day_10_part_2() {
        /*
        pretty_print_input(EXAMPLE_A);
        pretty_print_input(EXAMPLE_B);
        pretty_print_input(EXAMPLE_C);
        pretty_print_input(EXAMPLE_D);
        pretty_print_input(EXAMPLE_E);
        pretty_print_input(EXAMPLE_F);*/
        assert_eq!(day_10_part_2(EXAMPLE_A), 1);
        assert_eq!(day_10_part_2(EXAMPLE_B), 1);
        assert_eq!(day_10_part_2(EXAMPLE_C), 4);
        assert_eq!(day_10_part_2(EXAMPLE_D), 4);
        assert_eq!(day_10_part_2(EXAMPLE_E), 8);
        assert_eq!(day_10_part_2(EXAMPLE_F), 10);
    }
}
