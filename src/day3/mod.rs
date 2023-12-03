use std::fs;

pub fn day3_task1() -> i32 {
    let content = fs::read_to_string("./src/day3/input.txt").expect("Could not read file");
    let lines = content.lines();

    let column_count = lines.clone().count();
    let row_count = lines.skip(1).take(1).collect::<String>().chars().count();

    let mut chars: Vec<char> = content.chars().collect();
    let mut sum = 0;
    for y in 0..column_count {
        for x in 0..row_count {
            let char = chars[y * (row_count + 1) + x];
            if char.is_numeric() || char == '.' {
                continue;
            }
            let numbers = get_numbers_and_remove_number_from_list(
                &mut chars,
                x,
                y,
                row_count,
                content.chars().count(),
            );
            sum += numbers.iter().sum::<i32>();
        }
    }

    sum
}

pub fn day3_task2() -> i32 {
    let content = fs::read_to_string("./src/day3/input.txt").expect("Could not read file");
    let lines = content.lines();

    let column_count = lines.clone().count();
    let row_count = lines.skip(1).take(1).collect::<String>().chars().count();

    let mut chars: Vec<char> = content.chars().collect();
    let mut sum = 0;
    for y in 0..column_count {
        for x in 0..row_count {
            let char = chars[y * (row_count + 1) + x];
            if char.is_numeric() || char == '.' {
                continue;
            }
            let numbers = get_numbers_and_remove_number_from_list(
                &mut chars,
                x,
                y,
                row_count,
                content.chars().count(),
            );
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
    }

    sum
}

fn get_numbers_and_remove_number_from_list(
    chars: &mut [char],
    x: usize,
    y: usize,
    row_count: usize,
    total_count: usize,
) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for check_y in (y - 1)..(y + 2) {
        for check_x in (x - 1)..(x + 2) {
            if check_y * (row_count + 1) + check_x >= total_count {
                continue;
            }
            let char = chars[check_y * (row_count + 1) + check_x];
            if char.is_numeric() {
                numbers.push(get_number_and_replace(
                    chars,
                    check_y * (row_count + 1) + check_x,
                ));
            }
        }
    }

    numbers
}

fn get_number_and_replace(chars: &mut [char], start_pos: usize) -> i32 {
    let mut num_as_str = String::new();
    let mut pos = start_pos;
    while chars[pos].is_numeric() && pos > 0 {
        pos -= 1;
    }
    if chars[pos].is_numeric() {
        num_as_str.push(chars[pos]);
    }

    pos += 1;
    while chars[pos].is_numeric() {
        num_as_str.push(chars[pos]);
        chars[pos] = '.';
        pos += 1;
    }

    if num_as_str.is_empty() {
        return 0;
    }
    num_as_str.parse::<i32>().unwrap()
}
