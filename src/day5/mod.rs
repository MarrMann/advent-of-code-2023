use std::collections::HashSet;
use std::fmt::Formatter;
use std::{fmt, fs, str::Lines};

struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{{{}, {}, {}}}",
            self.destination_range_start, self.source_range_start, self.range_length
        )
    }
}

pub fn day5_task1() -> u64 {
    let contents = fs::read_to_string("src/day5/input.txt").expect("Could not read input file");
    let lines = contents.lines();

    let mut seeds = get_seeds(lines.clone());
    let maps = get_all_maps(lines.collect::<Vec<&str>>()).expect("Could not parse maps");
    let mut changed_indicies: HashSet<i32> = HashSet::new();

    maps.into_iter().for_each(|map| {
        let range = seeds.clone();
        let mut new_range: Vec<u64>;
        let mut final_range = range.clone();
        for submap in &map {
            new_range = transform_range_to_next(range.clone(), submap);
            for i in 0..range.len() {
                if range[i] != new_range[i] {
                    final_range[i] = new_range[i];
                }
            }
        }
        seeds = final_range;
        changed_indicies.clear();
    });

    *seeds.iter().min().unwrap()
}

fn get_seeds(lines: Lines<'_>) -> Vec<u64> {
    let seed_line = lines
        .clone()
        .next()
        .expect("Could not read first line of input");
    let seed_line = seed_line.split(": ").collect::<Vec<&str>>()[1];
    seed_line
        .split(' ')
        .map(|x| x.parse::<u64>().expect("Could not parse seed"))
        .collect()
}

fn get_all_maps(lines: Vec<&str>) -> Option<Vec<Vec<Map>>> {
    let mut maps: Vec<Vec<Map>> = Vec::new();
    maps.push(Vec::new());
    let iter = lines.iter();
    let iter = iter.skip(2);
    let mut map_index = 0;

    for line in iter {
        let char = line.chars().next();
        if char.is_none() {
            maps.push(Vec::new());
            map_index += 1;
            continue;
        }
        if !char.unwrap().is_numeric() {
            continue;
        }

        let parts = line.split(' ').collect::<Vec<&str>>();
        maps[map_index].push(Map {
            destination_range_start: parts[0]
                .parse::<u64>()
                .expect("Could not parse destination range start"),
            source_range_start: parts[1]
                .parse::<u64>()
                .expect("Could not parse source range start"),
            range_length: parts[2]
                .parse::<u64>()
                .expect("Could not parse range length"),
        });
    }

    if maps.is_empty() {
        return None;
    }
    Some(maps)
}

fn transform_range_to_next(range: Vec<u64>, map: &Map) -> Vec<u64> {
    let mut new_range: Vec<u64> = Vec::new();
    for number in range.clone() {
        if number > map.source_range_start && number < map.source_range_start + map.range_length {
            new_range.push(map.destination_range_start + (number - map.source_range_start));
            continue;
        }
        new_range.push(number);
    }
    new_range
}
