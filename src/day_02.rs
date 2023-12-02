#[derive(Debug)]
struct GameReveal {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    id: i64,
    reveals: Vec<GameReveal>,
}

type Games = Vec<Game>;

fn parse_input_data(input: &str) -> Games {
    return input
        .lines()
        .map(|line| {
            // return error if line doesn't start with "Game "
            if !line.starts_with("Game ") {
                panic!("Invalid input data, game doesn't start with game");
            }
            // Find the location of the :
            let colon_index = line
                .find(':')
                .expect("Invalid input data, can't find game separator");
            // Get the game id
            let game_id = line[5..colon_index]
                .parse::<i64>()
                .expect("Invalid input data, can't find game id");

            let reveals = line[colon_index + 1..]
                .split(';')
                .map(|reveal| {
                    let mut game_reveal = GameReveal {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    for nb_dices_per_colour in reveal.split(',') {
                        if nb_dices_per_colour.ends_with("red") {
                            game_reveal.red = nb_dices_per_colour[1..nb_dices_per_colour.len() - 4]
                                .parse::<u8>()
                                .expect("Invalid input data, can't parse red dice");
                        } else if nb_dices_per_colour.ends_with("green") {
                            game_reveal.green = nb_dices_per_colour
                                [1..nb_dices_per_colour.len() - 6]
                                .parse::<u8>()
                                .expect("Invalid input data, can't parse green dice");
                        } else if nb_dices_per_colour.ends_with("blue") {
                            game_reveal.blue = nb_dices_per_colour
                                [1..nb_dices_per_colour.len() - 5]
                                .parse::<u8>()
                                .expect("Invalid input data, can't parse blue dice");
                        } else {
                            panic!("Invalid input data, can't parse dice");
                        }
                    }
                    return game_reveal;
                })
                .collect();

            return Game {
                id: game_id,
                reveals: reveals,
            };
        })
        .collect();
}

pub fn day_2_part_1(data: &str) -> i64 {
    let games = parse_input_data(data);

    games
        .iter()
        .map(|game| {
            for reveal in &game.reveals {
                // only 12 red cubes, 13 green cubes, and 14 blue cubes?
                if reveal.red > 12 || reveal.green > 13 || reveal.blue > 14 {
                    return 0;
                }
            }
            return game.id;
        })
        .sum()
}

pub fn day_2_part_2(data: &str) -> i64 {
    let games = parse_input_data(data);

    games
        .iter()
        .map(|game| {
            let mut maximums = GameReveal {
                red: 0,
                green: 0,
                blue: 0,
            };
            for reveal in &game.reveals {
                if reveal.red > maximums.red {
                    maximums.red = reveal.red;
                }
                if reveal.green > maximums.green {
                    maximums.green = reveal.green;
                }
                if reveal.blue > maximums.blue {
                    maximums.blue = reveal.blue;
                }
            }
            return maximums.red as i64 * maximums.green as i64 * maximums.blue as i64;
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_day_2_part_1() {
        assert_eq!(day_2_part_1(EXAMPLE), 8);
    }

    #[test]
    fn test_day_2_part_2() {
        assert_eq!(day_2_part_2(EXAMPLE), 2286);
    }
}
