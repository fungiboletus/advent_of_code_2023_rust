mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

mod year_2015;

fn main() {
    let day_2015_12_01_data = include_str!("../inputs/year_2015/day_2015_12_01.txt");
    println!(
        "Day 2015, 12, part 1: {}",
        year_2015::day_2015_12_01::day_2015_12_01_part_1(day_2015_12_01_data)
    );
    println!(
        "Day 2015, 12, part 2: {}",
        year_2015::day_2015_12_01::day_2015_12_01_part_2(day_2015_12_01_data)
    );

    let day_2015_12_02_data = include_str!("../inputs/year_2015/day_2015_12_02.txt");
    println!(
        "Day 2015, 12, part 1: {}",
        year_2015::day_2015_12_02::day_2015_12_02_part_1(day_2015_12_02_data)
    );
    println!(
        "Day 2015, 12, part 2: {}",
        year_2015::day_2015_12_02::day_2015_12_02_part_2(day_2015_12_02_data)
    );

    let day_1_data = include_str!("../inputs/day_01.txt");
    println!("Day 1, part 1: {}", day_01::day_1_part_1(day_1_data));
    println!("Day 1, part 2: {}", day_01::day_1_part_2(day_1_data));

    let day_2_data = include_str!("../inputs/day_02.txt");
    println!("Day 2, part 1: {}", day_02::day_2_part_1(day_2_data));
    println!("Day 2, part 2: {}", day_02::day_2_part_2(day_2_data));

    let day_3_data = include_str!("../inputs/day_03.txt");
    println!("Day 3, part 1: {}", day_03::day_3_part_1(day_3_data));
    println!("Day 3, part 2: {}", day_03::day_3_part_2(day_3_data));

    let day_4_data = include_str!("../inputs/day_04.txt");
    println!("Day 4, part 1: {}", day_04::day_4_part_1(day_4_data));
    println!("Day 4, part 2: {}", day_04::day_4_part_2(day_4_data));

    let day_5_data = include_str!("../inputs/day_05.txt");
    println!("Day 5, part 1: {}", day_05::day_5_part_1(day_5_data));
    println!("Day 5, part 2: {}", day_05::day_5_part_2(day_5_data));

    let day_6_data = include_str!("../inputs/day_06.txt");
    println!("Day 6, part 1: {}", day_06::day_6_part_1(day_6_data));
    println!("Day 6, part 2: {}", day_06::day_6_part_2(day_6_data));

    let day_7_data = include_str!("../inputs/day_07.txt");
    println!("Day 7, part 1: {}", day_07::day_7_part_1(day_7_data));
    println!("Day 7, part 2: {}", day_07::day_7_part_2(day_7_data));

    let day_8_data = include_str!("../inputs/day_08.txt");
    println!("Day 8, part 1: {}", day_08::day_8_part_1(day_8_data));
    println!("Day 8, part 2: {}", day_08::day_8_part_2(day_8_data));

    let day_9_data = include_str!("../inputs/day_09.txt");
    println!("Day 9, part 1: {}", day_09::day_9_part_1(day_9_data));
    println!("Day 9, part 2: {}", day_09::day_9_part_2(day_9_data));
}
