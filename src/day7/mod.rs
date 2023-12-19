use std::{collections::HashMap, fs};

#[derive(Eq, Debug, Ord, PartialEq, PartialOrd)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: u32,
    rank: Rank,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank == other.rank {
            for c in 0..self.cards.len() {
                let self_val = map_card(self.cards.chars().nth(c).unwrap());
                let other_val = map_card(other.cards.chars().nth(c).unwrap());
                if self_val == other_val {
                    continue;
                }
                return self_val.cmp(&other_val);
            }
            return self.bid.cmp(&other.bid);
        }
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand2 {
    cards: String,
    bid: u32,
    rank: Rank,
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank == other.rank {
            for c in 0..self.cards.len() {
                let self_val = map_card_task2(self.cards.chars().nth(c).unwrap());
                let other_val = map_card_task2(other.cards.chars().nth(c).unwrap());
                if self_val == other_val {
                    continue;
                }
                return self_val.cmp(&other_val);
            }
            return self.bid.cmp(&other.bid);
        }
        self.rank.cmp(&other.rank)
    }
}

pub fn task1() -> u64 {
    let contents = fs::read_to_string("src/day7/input.txt").expect("Could not read input file");
    let mut hands = contents.lines().map(get_hand).collect::<Vec<Hand>>();
    hands.sort();

    let mut total: u64 = 0;
    (0..hands.len()).for_each(|i| {
        total += hands[i].bid as u64 * (i as u64 + 1);
    });

    total
}

pub fn task2() -> u64 {
    let contents = fs::read_to_string("src/day7/input.txt").expect("Could not read input file");
    let mut hands = contents.lines().map(get_hand_task2).collect::<Vec<Hand2>>();
    hands.sort();

    let mut total: u64 = 0;
    (0..hands.len()).for_each(|i| {
        total += hands[i].bid as u64 * (i as u64 + 1);
    });

    total
}

fn map_card(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn map_card_task2(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        'J' => 1,
        _ => card.to_digit(10).unwrap(),
    }
}

fn get_hand(input: &str) -> Hand {
    let split = input.split(' ').collect::<Vec<&str>>();

    Hand {
        cards: split[0].to_string(),
        bid: split[1].parse::<u32>().unwrap(),
        rank: get_rank(split[0].to_string()),
    }
}

fn get_hand_task2(input: &str) -> Hand2 {
    let split = input.split(' ').collect::<Vec<&str>>();
    Hand2 {
        cards: split[0].to_string(),
        bid: split[1].parse::<u32>().unwrap(),
        rank: get_rank_task2(split[0].to_string()),
    }
}

fn get_rank(hand: String) -> Rank {
    let mut hash_map: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        let count = hash_map.get(&c).unwrap_or(&0);
        hash_map.insert(c, count + 1);
    }
    let highest = hash_map.iter().max_by_key(|x| x.1).unwrap();
    let second_highest = hash_map
        .iter()
        .filter(|x| x.0 != highest.0)
        .max_by_key(|x| x.1)
        .unwrap_or((&'0', &0));

    get_rank_from_matches(*highest.1, *second_highest.1)
}

fn get_rank_task2(hand: String) -> Rank {
    let mut hash_map: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        let count = hash_map.get(&c).unwrap_or(&0);
        hash_map.insert(c, count + 1);
    }
    let highest = hash_map
        .iter()
        .filter(|kvp| kvp.0 != &'J')
        .max_by_key(|x| x.1)
        .unwrap_or((&'0', &0));
    let second_highest = hash_map
        .iter()
        .filter(|x| x.0 != highest.0 && x.0 != &'J')
        .max_by_key(|x| x.1)
        .unwrap_or((&'0', &0));
    let jokers = hash_map.get(&'J').unwrap_or(&0);

    get_rank_from_matches(*highest.1 + *jokers, *second_highest.1)
}

fn get_rank_from_matches(highest: u32, second_highest: u32) -> Rank {
    match highest {
        5 => Rank::FiveOfAKind,
        4 => Rank::FourOfAKind,
        3 => {
            if second_highest == 2 {
                Rank::FullHouse
            } else {
                Rank::ThreeOfAKind
            }
        }
        2 => {
            if second_highest == 2 {
                Rank::TwoPairs
            } else {
                Rank::OnePair
            }
        }
        _ => Rank::HighCard,
    }
}
