mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;

fn execute_day<F, G>(day: &str, data: &str, part_1: F, part_2: G) -> ()
where
    F: Fn(&str) -> i64,
    G: Fn(&str) -> i64,
{
    let now = std::time::Instant::now();
    println!("Day {}, part 1: {}", day, part_1(data));
    println!("Day {}, part 2: {}", day, part_2(data));
    println!("Time: {:?}", now.elapsed());
}

macro_rules! execute_day {
    ($day:expr, $part_1:expr, $part_2:expr) => {
        execute_day(
            $day,
            include_str!(concat!("../inputs/day_", $day, ".txt")),
            $part_1,
            $part_2,
        );
    };
}

fn main() {
    execute_day!("01", day_01::day_1_part_1, day_01::day_1_part_2);
    execute_day!("02", day_02::day_2_part_1, day_02::day_2_part_2);
    execute_day!("03", day_03::day_3_part_1, day_03::day_3_part_2);
    execute_day!("04", day_04::day_4_part_1, day_04::day_4_part_2);
    execute_day!("05", day_05::day_5_part_1, day_05::day_5_part_2);
    execute_day!("06", day_06::day_6_part_1, day_06::day_6_part_2);
    execute_day!("07", day_07::day_7_part_1, day_07::day_7_part_2);
    execute_day!("08", day_08::day_8_part_1, day_08::day_8_part_2);
    execute_day!("09", day_09::day_9_part_1, day_09::day_9_part_2);
    execute_day!("10", day_10::day_10_part_1, day_10::day_10_part_2);
    execute_day!("11", day_11::day_11_part_1, day_11::day_11_part_2);
    execute_day!("12", day_12::day_12_part_1, day_12::day_12_part_2);
    execute_day!("13", day_13::day_13_part_1, day_13::day_13_part_2);
    execute_day!("14", day_14::day_14_part_1, day_14::day_14_part_2);
    execute_day!("15", day_15::day_15_part_1, day_15::day_15_part_2);
    execute_day!("16", day_16::day_16_part_1, day_16::day_16_part_2);
    execute_day!("17", day_17::day_17_part_1, day_17::day_17_part_2);
    execute_day!("18", day_18::day_18_part_1, day_18::day_18_part_2);
}
