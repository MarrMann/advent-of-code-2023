use std::fs;

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

struct GameResult {
    game_number: i32,
    max_red_cubes: i32,
    max_green_cubes: i32,
    max_blue_cubes: i32,
}

pub fn day2_task1() -> i32 {
    let content =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");
    let games = content.lines();

    let mut sum = 0;
    for game in games {
        let game_result = get_game_max_results(game.to_string());
        if game_result.max_red_cubes <= MAX_RED_CUBES
            && game_result.max_green_cubes <= MAX_GREEN_CUBES
            && game_result.max_blue_cubes <= MAX_BLUE_CUBES
        {
            sum += game_result.game_number;
        }
    }

    sum
}

pub fn day2_task2() -> i32 {
    let content =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");
    let games = content.lines();

    let mut sum = 0;
    for game in games {
        let game_result = get_game_max_results(game.to_string());
        sum += game_result.max_red_cubes * game_result.max_green_cubes * game_result.max_blue_cubes;
    }

    sum
}

fn get_game_max_results(game: String) -> GameResult {
    let game_parts: Vec<&str> = game.split(": ").collect();
    let parts: Vec<&str> = game_parts[1].split("; ").collect();

    let mut max_reds: i32 = 0;
    let mut max_greens: i32 = 0;
    let mut max_blues: i32 = 0;

    for part in parts {
        let cubes: Vec<&str> = part.split(", ").collect();
        for cube in cubes {
            let split: Vec<&str> = cube.split(' ').collect();
            match split[1] {
                "red" => {
                    if split[0].parse::<i32>().unwrap() > max_reds {
                        max_reds = split[0].parse::<i32>().unwrap();
                    }
                }
                "green" => {
                    if split[0].parse::<i32>().unwrap() > max_greens {
                        max_greens = split[0].parse::<i32>().unwrap();
                    }
                }
                "blue" => {
                    if split[0].parse::<i32>().unwrap() > max_blues {
                        max_blues = split[0].parse::<i32>().unwrap();
                    }
                }
                &_ => {}
            }
        }
    }

    let game_number_string: Vec<&str> = game_parts[0].split(' ').collect();
    let game_number = game_number_string[1].parse::<i32>().unwrap();

    GameResult {
        game_number,
        max_red_cubes: max_reds,
        max_green_cubes: max_greens,
        max_blue_cubes: max_blues,
    }
}
