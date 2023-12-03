use std::fs;

const NUMBER_CONVERSION_TABLE: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn day1_task1() -> String {
    let contents =
        fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut sum: i32 = 0;
    for line in lines {
        let mut num_string: String = "".to_string();
        for char in line.chars() {
            if char.is_numeric() {
                num_string.push(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                num_string.push(char);
                break;
            }
        }
        sum += num_string
            .parse::<i32>()
            .expect("String should be a number");
    }

    sum.to_string()
}

pub fn day1_task2() -> String {
    let contents =
        fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut sum: i32 = 0;
    for mut line in lines {
        let parsed = parse_to_numbers_string(line.to_string());
        line = parsed.as_str();
        let mut num_string: String = "".to_string();
        for char in line.chars() {
            if char.is_numeric() {
                num_string.push(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                num_string.push(char);
                break;
            }
        }
        sum += num_string
            .parse::<i32>()
            .expect("String should be a number");
    }

    sum.to_string()
}

fn parse_to_numbers_string(input: String) -> String {
    // First, find all occurences of a number as a word, and save the index where it occues
    let mut matches: Vec<(usize, &str)> = Vec::new();
    for (_, number) in NUMBER_CONVERSION_TABLE.iter().enumerate() {
        let mut word_matches: Vec<_> = input.match_indices(number).collect();
        matches.append(&mut word_matches);
    }
    matches.sort_by_key(|a| a.0);

    // Next, convert all the words to numbers and add them PLUS the any actual numbers in order.
    let mut result: String = "".to_string();
    for (i, char) in input.chars().enumerate() {
        if char.is_numeric() {
            result.push(char);
        } else if !matches.is_empty() && i == matches[0].0 {
            result.push_str(
                NUMBER_CONVERSION_TABLE
                    .iter()
                    .position(|&r| r == matches[0].1)
                    .expect("Should have been able to find the number")
                    .to_string()
                    .as_str(),
            );

            matches.remove(0);
        }
    }

    result
}
