use std::fs;

pub fn task1() -> u32 {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Could not read input file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let times = get_numbers_for_line(lines[0]);
    let distances = get_numbers_for_line(lines[1]);

    let winning_numbers = get_winning_numbers(times[0], distances[0]);
    let mut total = winning_numbers.1 - winning_numbers.0;

    for i in 1..times.len() {
        let winning_numbers = get_winning_numbers(times[i], distances[i]);
        total *= winning_numbers.0 - winning_numbers.1;
    }

    total
}

fn get_winning_numbers(time: u32, distance: u32) -> (u32, u32) {
    let mut first = 0;
    for i in 0..time {
        let result = (time - i) * i;
        if result > distance {
            first = result;
        }
    }

    let mut second = 0;
    for i in (0..time).rev() {
        let result = (time - i) * i;
        if result > distance {
            second = result;
        }
    }

    (first, second)
}

fn get_numbers_for_line(line: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let line_chars = line.chars().collect::<Vec<char>>();

    for mut i in 0..line.chars().count() {
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
    }

    println!("Numbers: {:?}", numbers);

    numbers
}
