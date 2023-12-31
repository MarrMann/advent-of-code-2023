mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("Advent of Code 2023 solutions:");
    println!(
        "Day 1, task 1: {} | task 2: {}",
        day1::day1_task1(),
        day1::day1_task2()
    );
    println!(
        "Day 2, task 1: {} | task 2: {}",
        day2::day2_task1(),
        day2::day2_task2()
    );
    // println!(
    //     "Day 3, task 1: {} | task 2: {}",
    //     day3::day3_task1(),
    //     day3::day3_task2()
    // );
    println!(
        "Day 4, task 1: {} | task 2: {}",
        day4::day4_task1(),
        day4::day4_task2()
    );
    println!("Day 5, task 1: {}", day5::day5_task1());
    println!(
        "Day 6, task 1: {} | task 2: {}",
        day6::task1(),
        day6::task2()
    );
    println!(
        "Day 7, task 1: {} | task 2: {}",
        day7::task1(),
        day7::task2()
    );
}
