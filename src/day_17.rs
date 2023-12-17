/*
    Part1: Today looks like djistra pathfinding with a twist.

    The twist is annoying.
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use ndarray::Array2;
use nom::{
    character::{
        complete::{line_ending, satisfy},
        is_digit,
    },
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn parse_input_data(data: &str) -> IResult<&str, Array2<u8>> {
    map(
        separated_list1(line_ending, many1(satisfy(|c| is_digit(c as u8)))),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col] as u8 - 48)
        },
    )(data)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct Straight {
    direction: Direction,
    straight_length: usize,
}

impl Straight {
    fn is_valid(&self) -> bool {
        self.straight_length < 4
    }

    fn go_to(&self, direction: Direction) -> Straight {
        if self.direction == direction {
            Straight {
                direction,
                straight_length: self.straight_length + 1,
            }
        } else {
            Straight {
                direction,
                straight_length: 1,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct CostlyTile {
    row: usize,
    col: usize,
    straight: Straight,
}

fn dijkstra(grid: &Array2<u8>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let dim = grid.dim();
    let (nrows, ncols) = dim;
    let mut costs: HashMap<CostlyTile, usize> = HashMap::new();

    let mut priority_queue: BinaryHeap<Reverse<(usize, CostlyTile)>> = BinaryHeap::new();

    // In practice, going right and down should be enough
    // as we start from the top left corner.
    // And only right works on my input.
    // I think it could have been possible to do it
    // a bit more consily, but I got off by one errors
    // a bit too many times.
    priority_queue.push(Reverse((
        grid[[start.0, start.1 + 1]] as usize,
        CostlyTile {
            row: start.0,
            col: start.1 + 1,
            straight: Straight {
                direction: Direction::Right,
                straight_length: 2,
            },
        },
    )));
    priority_queue.push(Reverse((
        grid[[start.0 + 1, start.1]] as usize,
        CostlyTile {
            row: start.0 + 1,
            col: start.1,
            straight: Straight {
                direction: Direction::Down,
                straight_length: 2,
            },
        },
    )));

    while let Some(Reverse((cost, costly_tile))) = priority_queue.pop() {
        let CostlyTile { row, col, straight } = costly_tile;
        let Straight {
            direction,
            straight_length: _,
        } = straight;

        if (row, col) == end {
            return Some(cost);
        }

        if cost >= *costs.get(&costly_tile).unwrap_or(&usize::MAX) {
            continue;
        }

        costs.insert(costly_tile, cost);

        // If we can go up
        if row > 0 && direction != Direction::Down {
            let next_straight = straight.go_to(Direction::Up);
            if next_straight.is_valid() {
                let next_cost = cost + grid[[row - 1, col]] as usize;
                let next_costly_tile = CostlyTile {
                    row: row - 1,
                    col,
                    straight: next_straight,
                };
                if next_cost < *costs.get(&next_costly_tile).unwrap_or(&usize::MAX) {
                    priority_queue.push(Reverse((next_cost, next_costly_tile)));
                }
            }
        }
        // If we can go down
        if row < nrows - 1 && direction != Direction::Up {
            let next_straight = straight.go_to(Direction::Down);
            if next_straight.is_valid() {
                let next_cost = cost + grid[[row + 1, col]] as usize;
                let next_costly_tile = CostlyTile {
                    row: row + 1,
                    col,
                    straight: next_straight,
                };
                if next_cost < *costs.get(&next_costly_tile).unwrap_or(&usize::MAX) {
                    priority_queue.push(Reverse((next_cost, next_costly_tile)));
                }
            }
        }
        // If we can go left
        if col > 0 && direction != Direction::Right {
            let next_straight = straight.go_to(Direction::Left);
            if next_straight.is_valid() {
                let next_cost = cost + grid[[row, col - 1]] as usize;
                let next_costly_tile = CostlyTile {
                    row,
                    col: col - 1,
                    straight: next_straight,
                };
                if next_cost < *costs.get(&next_costly_tile).unwrap_or(&usize::MAX) {
                    priority_queue.push(Reverse((next_cost, next_costly_tile)));
                }
            }
        }
        // If we can go right
        if col < ncols - 1 && direction != Direction::Left {
            let next_straight = straight.go_to(Direction::Right);
            if next_straight.is_valid() {
                let next_cost = cost + grid[[row, col + 1]] as usize;
                let next_costly_tile = CostlyTile {
                    row,
                    col: col + 1,
                    straight: next_straight,
                };
                if next_cost < *costs.get(&next_costly_tile).unwrap_or(&usize::MAX) {
                    priority_queue.push(Reverse((next_cost, next_costly_tile)));
                }
            }
        }
    }

    None
}

pub fn day_17_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let start = (0, 0);
    let end = (grid.nrows() - 1, grid.ncols() - 1);
    dijkstra(&grid, start, end).expect("Failed to find a path") as i64
}

pub fn day_17_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const EXAMPLE_BUG: &str = "9333
9393
0110
9229";

    #[test]
    fn test_day_17_part_1() {
        assert_eq!(day_17_part_1(EXAMPLE), 102);
    }

    #[test]
    fn test_bug_part_1() {
        let (_, grid) = parse_input_data(EXAMPLE_BUG).expect("Failed to parse input data");
        assert_eq!(
            dijkstra(&grid, (2, 0), (2, 3)).expect("Failed to find a path"),
            6
        );
    }

    #[test]
    fn test_day_17_part_2() {
        assert_eq!(day_17_part_2(EXAMPLE), 42);
    }
}
