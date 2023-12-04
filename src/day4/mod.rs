use std::{fs, collections::HashMap};

pub fn day4_task1() -> u32 {
    let input = fs::read_to_string("src/day4/input.txt").expect("Could not read input file");
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let (winning_numbers, own_numbers) = split_into_winning_numbers_and_own_numbers(line);
        let winning_numbers = get_numbers_from_string(winning_numbers);
        let own_numbers = get_numbers_from_string(own_numbers);

        let mut correct_numbers = 0;
        for number in own_numbers {
            if winning_numbers.contains(&number) {
                correct_numbers += 1;
            }
        }
        if correct_numbers > 0 {
            sum += 2u32.pow(correct_numbers - 1);
        }
    }

    sum
}

pub fn day4_task2() -> u32 {
    let input = fs::read_to_string("src/day4/input.txt").expect("Could not read input file");
    let lines = input.lines();
    let mut scratch_cards: HashMap<u32, u32> = HashMap::from_iter((0..lines.clone().count()).map(|x| (x as u32, 1u32)));

    let mut index: u32 = 0;
    for line in lines {
        let current_scratch_card_instances = &scratch_cards.get(&index).unwrap().clone();
        index += 1;

        let (winning_numbers, own_numbers) = split_into_winning_numbers_and_own_numbers(line);
        let winning_numbers = get_numbers_from_string(winning_numbers);
        let own_numbers = get_numbers_from_string(own_numbers);

        let mut correct_numbers = 0;
        for number in own_numbers {
            if winning_numbers.contains(&number) {
                correct_numbers += 1;
            }
        }
        for number in 0..correct_numbers {
            let insert_index = index + number;
            scratch_cards.insert(insert_index, scratch_cards.get(&insert_index).unwrap() + current_scratch_card_instances);
        }
    }

    scratch_cards.into_iter().map(|v| v.1).sum()
}

fn split_into_winning_numbers_and_own_numbers(numbers: &str) -> (&str, &str) {
    let game_removed = numbers.split(": ").collect::<Vec<&str>>()[1];
    let numbers: Vec<&str> = game_removed.split(" | ").collect();
    (numbers[0], numbers[1])
}

fn get_numbers_from_string(numbers: &str) -> Vec<u32> {
    let mut numbers_vec = Vec::new();
    for i in (0..numbers.len()).step_by(3) {
        let number = numbers[i..i+2].to_string().replace(" ", "");
        numbers_vec.push(u32::from_str_radix(&number, 10).expect("Could not parse number"));
    }
    numbers_vec
}