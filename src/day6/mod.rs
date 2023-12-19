use std::fs;

pub fn task1() -> i64 {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Could not read input file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let times = get_numbers_for_line(lines[0]);
    let distances = get_numbers_for_line(lines[1]);
    let winning_numbers = get_winning_numbers(times[0], distances[0]);
    let mut total = winning_numbers.1 as i64 - winning_numbers.0 as i64;

    for i in 1..times.len() {
        let winning_numbers = get_winning_numbers(times[i], distances[i]);
        total *= winning_numbers.1 as i64 - winning_numbers.0 as i64;
    }

    total
}
pub fn task2() -> i64 {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Could not read input file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let time = get_numbers_ignore_spaces(lines[0]);
    let distance = get_numbers_ignore_spaces(lines[1]);
    let winning_numbers = get_winning_numbers_u64(time, distance);

    winning_numbers.1 as i64 - winning_numbers.0 as i64
}

fn get_winning_numbers(time: u32, distance: u32) -> (u32, u32) {
    let mut first = 0;
    for i in 1..time {
        let result = (time - i) * i;
        if result > distance {
            first = i;
            break;
        }
    }

    let mut second = 0;
    for i in (1..time).rev() {
        let result = (time - i) * i;
        if result > distance {
            second = i + 1;
            break;
        }
    }

    (first, second)
}

fn get_winning_numbers_u64(time: u64, distance: u64) -> (u64, u64) {
    let mut first = 0;
    for i in 1..time {
        let result = (time - i) * i;
        if result > distance {
            first = i;
            break;
        }
    }

    let mut second = 0;
    for i in (1..time).rev() {
        let result = (time - i) * i;
        if result > distance {
            second = i + 1;
            break;
        }
    }

    (first, second)
}

fn get_numbers_ignore_spaces(line: &str) -> u64 {
    let chars = line.chars().collect::<Vec<char>>();
    let mut char_num: String = "".to_string();

    for char in chars {
        if char.is_numeric() {
            char_num.push(char);
        }
    }

    char_num.parse::<u64>().expect("Could not parse number")
}

fn get_numbers_for_line(line: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let line_chars = line.chars().collect::<Vec<char>>();

    let mut i = 0;
    while i < line.chars().count() {
        let mut char = line_chars[i];
        let mut char_num: String = "".to_string();
        while char.is_numeric() {
            char_num.push(char);
            i += 1;
            if i >= line_chars.len() {
                break;
            }
            char = line_chars[i];
        }

        if !char_num.is_empty() {
            numbers.push(char_num.parse::<u32>().expect("Could not parse number"));
        }
        i += 1;
    }

    numbers
}
