/*
    Relatively simple problem that can be solved without any fancy algorithm.

    I tried to add some memoisation but the stack and the grid had to be memoised
    and it was simply not working.
*/
use ndarray::Array2;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    EmptySpace,         // .
    MirrorSlash,        // /
    MirrorBackslash,    // \
    SplitterHorizontal, // -
    SplitterVertical,   // |
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<Tile>> {
    map(
        separated_list1(
            line_ending,
            many1(map(one_of("./\\-|"), |c| match c {
                '.' => Tile::EmptySpace,
                '/' => Tile::MirrorSlash,
                '\\' => Tile::MirrorBackslash,
                '-' => Tile::SplitterHorizontal,
                '|' => Tile::SplitterVertical,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum VisitHeading {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct VisitSchedule {
    row: usize,
    col: usize,
    direction: VisitHeading,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Visit {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

#[allow(dead_code)]
fn pretty_print_visits(grid: &Array2<Tile>, visits: &Array2<Visit>) {
    for i in 0..visits.nrows() {
        for j in 0..visits.ncols() {
            let visit = visits[[i, j]];
            print!(
                "{}",
                match grid[[i, j]] {
                    Tile::MirrorSlash => '/',
                    Tile::MirrorBackslash => '\\',
                    Tile::SplitterHorizontal => '-',
                    Tile::SplitterVertical => '|',
                    Tile::EmptySpace => match (visit.up, visit.right, visit.down, visit.left) {
                        (false, false, false, false) => '.',
                        (true, false, false, false) => '^',
                        (false, true, false, false) => '>',
                        (false, false, true, false) => 'v',
                        (false, false, false, true) => '<',
                        _ => '#',
                    },
                }
            );
        }
        println!();
    }
    println!();
}

fn compute_beams(start: VisitSchedule, grid: &Array2<Tile>) -> Array2<Visit> {
    let (nb_rows, nb_cols) = grid.dim();

    let mut visits = Array2::from_elem((nb_rows, nb_cols), Visit::default());

    let mut stack: Vec<VisitSchedule> = Vec::new();
    stack.push(start);

    while let Some(visit) = stack.pop() {
        //pretty_print_visits(&grid, &visits);
        let tile = &grid[[visit.row, visit.col]];
        let previous_visit = visits[[visit.row, visit.col]];

        // Ignore if we already visited this tile from the same direction
        if match visit.direction {
            VisitHeading::Up => previous_visit.up,
            VisitHeading::Right => previous_visit.right,
            VisitHeading::Down => previous_visit.down,
            VisitHeading::Left => previous_visit.left,
        } {
            continue;
        }

        // Update the visits
        visits[[visit.row, visit.col]] = match visit.direction {
            VisitHeading::Up => Visit {
                up: true,
                ..previous_visit
            },
            VisitHeading::Right => Visit {
                right: true,
                ..previous_visit
            },
            VisitHeading::Down => Visit {
                down: true,
                ..previous_visit
            },
            VisitHeading::Left => Visit {
                left: true,
                ..previous_visit
            },
        };

        match tile {
            Tile::EmptySpace => match visit.direction {
                VisitHeading::Up => {
                    if visit.row > 0 {
                        stack.push(VisitSchedule {
                            row: visit.row - 1,
                            col: visit.col,
                            direction: VisitHeading::Up,
                        });
                    }
                }
                VisitHeading::Right => {
                    if visit.col < nb_cols - 1 {
                        stack.push(VisitSchedule {
                            row: visit.row,
                            col: visit.col + 1,
                            direction: VisitHeading::Right,
                        });
                    }
                }
                VisitHeading::Down => {
                    if visit.row < nb_rows - 1 {
                        stack.push(VisitSchedule {
                            row: visit.row + 1,
                            col: visit.col,
                            direction: VisitHeading::Down,
                        });
                    }
                }
                VisitHeading::Left => {
                    if visit.col > 0 {
                        stack.push(VisitSchedule {
                            row: visit.row,
                            col: visit.col - 1,
                            direction: VisitHeading::Left,
                        });
                    }
                }
            },
            Tile::MirrorSlash => {
                // /
                match visit.direction {
                    VisitHeading::Up => {
                        if visit.col < nb_cols - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col + 1,
                                direction: VisitHeading::Right,
                            });
                        }
                    }
                    VisitHeading::Right => {
                        if visit.row > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row - 1,
                                col: visit.col,
                                direction: VisitHeading::Up,
                            });
                        }
                    }
                    VisitHeading::Down => {
                        if visit.col > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col - 1,
                                direction: VisitHeading::Left,
                            });
                        }
                    }
                    VisitHeading::Left => {
                        if visit.row < nb_rows - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row + 1,
                                col: visit.col,
                                direction: VisitHeading::Down,
                            });
                        }
                    }
                }
            }
            Tile::MirrorBackslash => {
                // \
                match visit.direction {
                    VisitHeading::Up => {
                        if visit.col > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col - 1,
                                direction: VisitHeading::Left,
                            });
                        }
                    }
                    VisitHeading::Right => {
                        if visit.row < nb_rows - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row + 1,
                                col: visit.col,
                                direction: VisitHeading::Down,
                            });
                        }
                    }
                    VisitHeading::Down => {
                        if visit.col < nb_cols - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col + 1,
                                direction: VisitHeading::Right,
                            });
                        }
                    }
                    VisitHeading::Left => {
                        if visit.row > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row - 1,
                                col: visit.col,
                                direction: VisitHeading::Up,
                            });
                        }
                    }
                }
            }
            Tile::SplitterHorizontal => {
                // -
                match visit.direction {
                    // left and right
                    VisitHeading::Up | VisitHeading::Down => {
                        if visit.col > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col - 1,
                                direction: VisitHeading::Left,
                            });
                        }
                        if visit.col < nb_cols - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col + 1,
                                direction: VisitHeading::Right,
                            });
                        }
                    }
                    // passthrough
                    VisitHeading::Right => {
                        if visit.col < nb_cols - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col + 1,
                                direction: VisitHeading::Right,
                            });
                        }
                    }
                    // left and right
                    VisitHeading::Left => {
                        if visit.col > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row,
                                col: visit.col - 1,
                                direction: VisitHeading::Left,
                            });
                        }
                    }
                }
            }
            Tile::SplitterVertical => {
                // |
                match visit.direction {
                    // up and down
                    VisitHeading::Left | VisitHeading::Right => {
                        if visit.row > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row - 1,
                                col: visit.col,
                                direction: VisitHeading::Up,
                            });
                        }
                        if visit.row < nb_rows - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row + 1,
                                col: visit.col,
                                direction: VisitHeading::Down,
                            });
                        }
                    }
                    // passthrough
                    VisitHeading::Down => {
                        if visit.row < nb_rows - 1 {
                            stack.push(VisitSchedule {
                                row: visit.row + 1,
                                col: visit.col,
                                direction: VisitHeading::Down,
                            });
                        }
                    }
                    // up and down
                    VisitHeading::Up => {
                        if visit.row > 0 {
                            stack.push(VisitSchedule {
                                row: visit.row - 1,
                                col: visit.col,
                                direction: VisitHeading::Up,
                            });
                        }
                    }
                }
            }
        }
    }

    visits
}

trait VisitScheduleExt {
    fn compute_nb_visits(&self) -> usize;
}
impl VisitScheduleExt for Array2<Visit> {
    #[inline]
    fn compute_nb_visits(&self) -> usize {
        self.iter()
            .filter(|visit| visit.up || visit.down || visit.left || visit.right)
            .count()
    }
}

pub fn day_16_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");

    compute_beams(
        VisitSchedule {
            row: 0,
            col: 0,
            direction: VisitHeading::Right,
        },
        &grid,
    )
    .compute_nb_visits() as i64
}

pub fn day_16_part_2(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let (nb_rows, nb_cols) = grid.dim();

    (0..nb_cols)
        .map(|col| VisitSchedule {
            row: 0,
            col,
            direction: VisitHeading::Down,
        })
        .chain((0..nb_cols).map(|col| VisitSchedule {
            row: nb_rows - 1,
            col,
            direction: VisitHeading::Up,
        }))
        .chain((0..nb_rows).map(|row| VisitSchedule {
            row,
            col: 0,
            direction: VisitHeading::Right,
        }))
        .chain((0..nb_rows).map(|row| VisitSchedule {
            row,
            col: nb_cols - 1,
            direction: VisitHeading::Left,
        }))
        .par_bridge()
        .map(|schedule| compute_beams(schedule, &grid).compute_nb_visits())
        .max()
        .unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_day_16_part_1() {
        assert_eq!(day_16_part_1(EXAMPLE), 46);
    }

    #[test]
    fn test_day_16_part_2() {
        assert_eq!(day_16_part_2(EXAMPLE), 51);
    }
}
