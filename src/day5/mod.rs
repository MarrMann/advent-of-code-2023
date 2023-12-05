use std::{fs, str::Lines};

struct Map {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

pub fn day5_task1() -> u32 {
    let contents = fs::read_to_string("src/day5/input.txt").expect("Could not read input file");
    let mut lines = contents.lines();

    let mut seeds = get_seeds(lines.clone());
    for i in 0..lines.clone().count() {
        let line = lines.clone().nth(i).expect("Could not read line");
        if line.is_empty() {
            while let Some(maps) = get_next_maps( lines.clone().collect::<Vec<&str>>() ) {
                for map in maps {
                    seeds = transform_range_to_next(seeds, map);
                }
            }
        }
    }

    seeds.iter().min().unwrap().clone()
}

fn get_seeds(lines: Lines<'_>) -> Vec<u32> {
    let seed_line = lines.clone().nth(0).expect("Could not read first line of input");
    let seed_line = seed_line.split(": ").collect::<Vec<&str>>()[1];
    seed_line.split(" ").map(|x| x.parse::<u32>().expect("Could not parse seed")).collect()
}

fn get_next_maps(lines: Vec<&str>) -> Option<Vec<Map>> {
    let mut maps: Vec<Map> = Vec::new();

    while let Some(line) = lines.iter().next() {
        let char = line.chars().nth(0);
        if char == None || !char.unwrap().is_numeric() {
            if maps.len() > 0 {
                return Some(maps)
            }
            continue;
        }

        let parts = line.split(" ").collect::<Vec<&str>>();
        println!("parts: {:?}", parts);
        maps.push(
            Map { 
                destination_range_start: parts[0].parse::<u32>().expect("Could not parse destination range start"),
                source_range_start: parts[1].parse::<u32>().expect("Could not parse source range start"),
                range_length: parts[2].parse::<u32>().expect("Could not parse range length")
            }
        )
    }

    if maps.len() == 0 {
        return None
    }
    Some(maps)
}

fn transform_range_to_next(range: Vec<u32>, map: Map) -> Vec<u32> {
    let mut new_range: Vec<u32> = Vec::new();
    for number in range {
        if number > map.source_range_start && number < map.source_range_start + map.range_length {
            new_range.push(map.destination_range_start + (number - map.source_range_start));
            continue;
        }
        new_range.push(number);
    }
    new_range
}