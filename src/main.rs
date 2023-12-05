mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() {
    let day_1_data = include_str!("../inputs/day_01.txt");
    println!("Day 1, part 1: {}", day_01::day_1_part_1(day_1_data));
    println!("Day 1, part 2: {}", day_01::day_1_part_2(day_1_data));

    let day_2_data = include_str!("../inputs/day_02.txt");
    println!("Day 1, part 1: {}", day_02::day_2_part_1(day_2_data));
    println!("Day 1, part 2: {}", day_02::day_2_part_2(day_2_data));

    let day_3_data = include_str!("../inputs/day_03.txt");
    println!("Day 1, part 1: {}", day_03::day_3_part_1(day_3_data));
    println!("Day 1, part 2: {}", day_03::day_3_part_2(day_3_data));

    let day_4_data = include_str!("../inputs/day_04.txt");
    println!("Day 1, part 1: {}", day_04::day_4_part_1(day_4_data));
    println!("Day 1, part 2: {}", day_04::day_4_part_2(day_4_data));

    let day_5_data = include_str!("../inputs/day_05.txt");
    println!("Day 1, part 1: {}", day_05::day_5_part_1(day_5_data));
    println!("Day 1, part 2: {}", day_05::day_5_part_2(day_5_data));
}
