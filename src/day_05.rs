/*
    Day 5 took a while.

    I first struggled with nom, with some weird signature errors reminding me C++.
    I eventually made a simpler and less ambitious nom parser.

    The first part was easy, I just don't like to work on ranges.

    The second part was quite challenging. I didn't want to check for existing solutions
    because that kills the fun. As I am writing this comment before checking the other solutions,
    I believe that my solution is relatively complex and verbose.

    However I can see on the leaderboard that the fastest programmers still took a descent amount
    of time to solve it, so other solutions may be relatively complex too.

    I'm pretty happy with my solution as it runs pretty fast. I think it could be made slightly faster
    but this is good enough for me.

    I first expand the location maps to deal with the fact that some maps are implicits. I don't like that so
    everything is explicit. For every location, I have maps for everything between 0 up and the highest u64 value.

    I then compute more location maps to split everytime two locations has an intersection in their maps.
    I wanted to do that very early but I struggled to implement it well.

    Computing more maps will produce more intersections so I do that until the solution stabilise.

    I then have a map for each intersection, and we can now only check if a path exists by checking the existence
    of the destination in the other maps. I use a list of dictionaries to do that.

    I then check every location from the lowest to the highest and browse through everything in reverse order.
    I sort first to save some little CPU time.
*/

use std::{collections::HashMap, vec};

// I'm not sure nom makes me save time,
// but I keep practicing it.
use nom::{
    self,
    bytes::complete::tag,
    character::complete::{line_ending, none_of, space1},
    combinator::{map, recognize},
    combinator::{map_res, opt},
    error::ErrorKind,
    multi::{many0, many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct InputData {
    seeds: Vec<u64>,
    seed_to_soil_maps: Vec<Map>,
    soil_to_fertilizer_maps: Vec<Map>,
    fertilizer_to_water_maps: Vec<Map>,
    water_to_light_maps: Vec<Map>,
    light_to_temperature_maps: Vec<Map>,
    temperature_to_humidity_maps: Vec<Map>,
    humidity_to_location_maps: Vec<Map>,
}

fn parse_seeds(data: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            tag("seeds:"),
            space1,
            separated_list1(space1, nom::character::complete::u64),
        )),
        |(_, _, seeds)| seeds,
    )(data)
}

fn parse_map(data: &str) -> IResult<&str, Map> {
    map(
        tuple((
            nom::character::complete::u64,
            space1,
            nom::character::complete::u64,
            space1,
            nom::character::complete::u64,
        )),
        |(destination_range_start, _, source_range_start, _, range_length)| Map {
            destination_range_start,
            source_range_start,
            range_length,
        },
    )(data)
}

fn parse_maps(data: &str) -> IResult<&str, Vec<Map>> {
    many0(terminated(parse_map, opt(line_ending)))(data)
}

fn parse_maps_with_header(data: &str) -> IResult<&str, (&str, Vec<Map>)> {
    map(
        tuple((
            recognize(many1(none_of(" "))),
            space1,
            tag("map:"),
            line_ending,
            parse_maps,
        )),
        |(header, _, _, _, maps)| (header, maps),
    )(data)
}

fn parse_input_data(data: &str) -> IResult<&str, InputData> {
    map_res(
        tuple((
            parse_seeds,
            many1(line_ending),
            many0(terminated(parse_maps_with_header, opt(line_ending))),
        )),
        |(seeds, _, maps_and_headers)| {
            let mut input_data = InputData {
                seeds,
                seed_to_soil_maps: Vec::new(),
                soil_to_fertilizer_maps: Vec::new(),
                fertilizer_to_water_maps: Vec::new(),
                water_to_light_maps: Vec::new(),
                light_to_temperature_maps: Vec::new(),
                temperature_to_humidity_maps: Vec::new(),
                humidity_to_location_maps: Vec::new(),
            };

            for (header, maps) in maps_and_headers {
                match header {
                    "seed-to-soil" => input_data.seed_to_soil_maps = maps,
                    "soil-to-fertilizer" => input_data.soil_to_fertilizer_maps = maps,
                    "fertilizer-to-water" => input_data.fertilizer_to_water_maps = maps,
                    "water-to-light" => input_data.water_to_light_maps = maps,
                    "light-to-temperature" => input_data.light_to_temperature_maps = maps,
                    "temperature-to-humidity" => input_data.temperature_to_humidity_maps = maps,
                    "humidity-to-location" => input_data.humidity_to_location_maps = maps,
                    _ => {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            header,
                            ErrorKind::Tag,
                        )));
                    }
                }
            }

            return Ok(input_data);
        },
    )(data)
}

fn find_location(current_location: u64, maps: &Vec<Map>) -> u64 {
    for map in maps {
        if current_location >= map.source_range_start
            && current_location < map.source_range_start + map.range_length
        {
            let offset = current_location - map.source_range_start;
            return map.destination_range_start + offset;
        }
    }
    return current_location;
}

pub fn day_5_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Unable to parse input data");

    data.seeds
        .iter()
        .map(|seed| {
            let seed_location = *seed;
            let soil_location = find_location(seed_location, &data.seed_to_soil_maps);
            let fertilizer_location = find_location(soil_location, &data.soil_to_fertilizer_maps);
            let water_location = find_location(fertilizer_location, &data.fertilizer_to_water_maps);
            let light_location = find_location(water_location, &data.water_to_light_maps);
            let temperature_location =
                find_location(light_location, &data.light_to_temperature_maps);
            let humidity_location =
                find_location(temperature_location, &data.temperature_to_humidity_maps);
            let location = find_location(humidity_location, &data.humidity_to_location_maps);
            return location;
        })
        .min()
        .unwrap_or(0) as i64
}

// Fills the gaps in the maps, but apparently there is no gap
// unless at the beginning or at the end.
// still, I keep it.
fn expand_maps(maps: &Vec<Map>) -> Vec<Map> {
    // created a sorted copy of the maps
    let mut sorted_maps = maps.clone();
    sorted_maps.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));

    // println!("{:?}", sorted_maps);

    let mut expanded_maps = Vec::new();

    let mut current = 0;
    for map in sorted_maps {
        if map.source_range_start > current {
            expanded_maps.push(Map {
                destination_range_start: current,
                source_range_start: current,
                range_length: map.source_range_start - current,
            });
        }
        expanded_maps.push(map);
        current = map.source_range_start + map.range_length;
    }

    // fill the last gap
    let max = u64::max_value();
    if current < max {
        expanded_maps.push(Map {
            destination_range_start: current,
            source_range_start: current,
            range_length: max - current,
        });
    }

    // nicer to read
    expanded_maps.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));
    return expanded_maps;
}

// compute the maps that needs to be used to go from a source to a destination
// returns from and to
fn compute_useful_maps(from: &Vec<Map>, to: &Vec<Map>) -> (Vec<Map>, Vec<Map>) {
    // The concept of the algorithm is to go through the from and to map, and fill
    // two new lists of maps so there is always no gap.

    // sort from by destination and as a deque
    let mut sorted_from = from.clone();
    sorted_from.sort_by(|a, b| a.destination_range_start.cmp(&b.destination_range_start));
    // sort to by source
    let mut sorted_to = to.clone();
    sorted_to.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));

    let mut useful_from = Vec::new();
    let mut useful_to = Vec::new();

    for current_from_map in &sorted_from {
        let from_destination_start = current_from_map.destination_range_start;
        let from_destination_end =
            current_from_map.destination_range_start + current_from_map.range_length;

        let mut splits_to = vec![from_destination_start];

        for current_to_map in &sorted_to {
            let to_source_start = current_to_map.source_range_start;

            if to_source_start >= from_destination_start && to_source_start < from_destination_end {
                splits_to.push(to_source_start);
            }
        }

        splits_to.push(from_destination_end);

        let mut current_source_start = current_from_map.source_range_start;

        splits_to.windows(2).for_each(|window| {
            let a = window[0];
            let b = window[1];
            if a == b {
                // ignore duplicates
                return;
            }
            let range_length = b - a;
            useful_from.push(Map {
                destination_range_start: a,
                source_range_start: current_source_start,
                range_length,
            });
            current_source_start += range_length;
        });
    }

    for current_to_map in &sorted_to {
        let to_source_start = current_to_map.source_range_start;
        let to_source_end = current_to_map.source_range_start + current_to_map.range_length;

        let mut splits_from = vec![to_source_start];

        for current_from_map in &sorted_from {
            let from_destination_start = current_from_map.destination_range_start;

            if from_destination_start >= to_source_start && from_destination_start < to_source_end {
                splits_from.push(from_destination_start);
            }
        }

        splits_from.push(to_source_end);

        let mut current_destination_start = current_to_map.destination_range_start;

        splits_from.windows(2).for_each(|window| {
            let a = window[0];
            let b = window[1];
            if a == b {
                // ignore duplicates
                return;
            }
            let range_length = b - a;
            useful_to.push(Map {
                destination_range_start: current_destination_start,
                source_range_start: a,
                range_length,
            });
            current_destination_start += range_length;
        });
    }

    return (useful_from, useful_to);
}

fn compute_useful_maps_for_all_couples(couples: &Vec<Vec<Map>>) -> Vec<Vec<Map>> {
    let mut couples = couples.clone();
    loop {
        let mut has_a_change = false;
        for i in 0..couples.len() - 1 {
            let from = couples[i].clone();
            let to = couples[i + 1].clone();
            let (new_from, new_to) = compute_useful_maps(&from, &to);
            if new_from.len() != from.len() {
                couples[i] = new_from;
                has_a_change = true;
            }
            if new_to.len() != to.len() {
                couples[i + 1] = new_to;
                has_a_change = true;
            }
        }
        if !has_a_change {
            break;
        }
    }
    return couples;
}

fn has_road(start_place: u64, destinations: &Vec<HashMap<u64, u64>>) -> bool {
    let mut current_place = start_place;
    for destination in destinations.iter().rev() {
        if let Some(next_place) = destination.get(&current_place) {
            current_place = *next_place;
        } else {
            return false;
        }
    }
    return true;
}

fn find_lowest_location(destinations: &Vec<HashMap<u64, u64>>) -> Result<u64, &str> {
    let last_destination = destinations.last();
    if last_destination.is_none() {
        return Err("No destination");
    }
    let mut last_destinations_vec: Vec<u64> = last_destination.unwrap().keys().cloned().collect();
    // sort to save CPU time
    last_destinations_vec.sort();
    for destination in last_destinations_vec {
        if has_road(destination, destinations) {
            return Ok(destination);
        }
    }
    return Err("No destination");
}

pub fn day_5_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Unable to parse input data");

    let seeds_as_maps = data
        .seeds
        .chunks_exact(2)
        .map(|seed| Map {
            destination_range_start: seed[0],
            source_range_start: 0xdeadbeef,
            range_length: seed[1],
        })
        .collect::<Vec<Map>>();

    let maps = vec![
        data.seed_to_soil_maps,
        data.soil_to_fertilizer_maps,
        data.fertilizer_to_water_maps,
        data.water_to_light_maps,
        data.light_to_temperature_maps,
        data.temperature_to_humidity_maps,
        data.humidity_to_location_maps,
    ];

    let expanded_maps = maps
        .iter()
        .map(|maps| expand_maps(maps))
        .collect::<Vec<Vec<Map>>>();

    let mut couples = vec![
        // the seeds are not expanded
        seeds_as_maps,
    ];
    couples.extend(expanded_maps.into_iter());

    let new_couples = compute_useful_maps_for_all_couples(&couples);

    let destinations_as_dictionary = new_couples
        .iter()
        .map(|maps| {
            maps.iter()
                .map(|map| (map.destination_range_start, map.source_range_start))
                .collect::<HashMap<u64, u64>>()
        })
        .collect::<Vec<HashMap<u64, u64>>>();

    let smallest = find_lowest_location(&destinations_as_dictionary);
    return smallest.unwrap() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    /*fn pretty_print_maps(label: &str, maps: &Vec<Map>) {
        println!("{} map:", label);
        for map in maps {
            println!(
                "{} {} {}",
                map.destination_range_start, map.source_range_start, map.range_length
            );
        }
    }*/

    #[test]
    fn test_day_5_part_1() {
        assert_eq!(day_5_part_1(EXAMPLE), 35);
    }

    #[test]
    fn test_day_5_part_2() {
        assert_eq!(day_5_part_2(EXAMPLE), 46);
    }

    #[test]
    fn test_expand_maps_1() {
        let expanded = expand_maps(&vec![
            Map {
                destination_range_start: 60,
                source_range_start: 56,
                range_length: 37,
            },
            Map {
                destination_range_start: 56,
                source_range_start: 93,
                range_length: 4,
            },
        ]);

        assert!(expanded.iter().eq(&vec![
            Map {
                destination_range_start: 0,
                source_range_start: 0,
                range_length: 56
            },
            Map {
                destination_range_start: 60,
                source_range_start: 56,
                range_length: 37
            },
            Map {
                destination_range_start: 56,
                source_range_start: 93,
                range_length: 4
            },
            Map {
                destination_range_start: 97,
                source_range_start: 97,
                range_length: 18446744073709551518
            },
        ]));
    }

    #[test]
    fn test_expand_maps_2() {
        let expanded = expand_maps(&vec![
            Map {
                destination_range_start: 0,
                source_range_start: 69,
                range_length: 1,
            },
            Map {
                destination_range_start: 1,
                source_range_start: 0,
                range_length: 69,
            },
        ]);

        assert!(expanded.iter().eq(&vec![
            Map {
                destination_range_start: 1,
                source_range_start: 0,
                range_length: 69
            },
            Map {
                destination_range_start: 0,
                source_range_start: 69,
                range_length: 1
            },
            Map {
                destination_range_start: 70,
                source_range_start: 70,
                range_length: 18446744073709551545
            },
        ]));
    }
}
