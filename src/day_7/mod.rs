use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]

enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullhHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

impl HandType {
    fn from(hand: &str) -> Self {
        let mut map: HashMap<char, u32> = HashMap::new();
        for c in hand.chars() {
            let count = if let Some(count) = map.get(&c) {
                count + 1
            } else {
                1
            };
            map.insert(c, count);
        }
        let mut count_vec: Vec<(&char, &u32)> = map.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        let max = count_vec[0];

        let second_max = if count_vec.len() > 1 {
            count_vec[1]
        } else {
            (&'z', &0)
        };
        return match *max.1 {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => {
                if *second_max.1 == 2 {
                    HandType::FullhHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if *second_max.1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            _ => Self::HighCard,
        };
    }

    fn from_with_wildcard(hand: &str, wild: char) -> Self {
        let mut map: HashMap<char, u32> = HashMap::new();
        for c in hand.chars() {
            let count = if let Some(count) = map.get(&c) {
                count + 1
            } else {
                1
            };
            map.insert(c, count);
        }
        let mut count_vec: Vec<(&char, &u32)> = map.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut max = count_vec[0];

        let second_max = if count_vec.len() > 1 {
            count_vec[1]
        } else {
            (&'z', &0)
        };

        let mut new_val;
        if *max.0 == wild {
            new_val = *max.1 + *second_max.1;
            new_val = new_val.min(5);
            max.1 = &(new_val)
        } else {
            let wild_count = *map.get(&wild).unwrap_or(&0);
            new_val = *max.1 + wild_count;
            new_val = new_val.min(5);
            max.1 = &(new_val)
        }

        return match *max.1 {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => {
                if *second_max.1 == 2 {
                    HandType::FullhHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if *second_max.1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            _ => Self::HighCard,
        };
    }
}

#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    bid: u32,
    hand_type: HandType,
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str, bid: u32) -> Self {
        Hand {
            hand,
            bid,
            hand_type: HandType::from(hand),
        }
    }
    fn new_with_wildcard(hand: &'a str, bid: u32, wild: char) -> Self {
        Hand {
            hand,
            bid,
            hand_type: HandType::from_with_wildcard(hand, wild),
        }
    }

    fn compare(&'a self, other: &'a Hand, wild: Option<char>) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type < other.hand_type {
            Ordering::Less
        } else {
            let mut a_iter = self.hand.chars();
            let mut b_iter = other.hand.chars();
            loop {
                let a = match a_iter.next() {
                    Some(c) => c,
                    None => break,
                };
                let b = match b_iter.next() {
                    Some(c) => c,
                    None => break,
                };

                let val_a;
                let val_b;

                if let Some(c) = wild {
                    val_a = if c == a { 0 } else { get_val(a) };
                    val_b = if c == b { 0 } else { get_val(b) };
                } else {
                    val_a = get_val(a);
                    val_b = get_val(b);
                }

                if val_a > val_b {
                    return Ordering::Greater;
                } else if val_a < val_b {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_7.txt").unwrap();

    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, num_str) = line.split_once(" ").unwrap();
        hands.push(Hand::new(hand, num_str.parse().unwrap()))
    }

    hands.sort_by(|hand_1, hand_2| hand_1.compare(hand_2, None));

    let res = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + hand.bid * (index as u32 + 1));

    println!("res = {}", res);
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_7.txt").unwrap();

    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, num_str) = line.split_once(" ").unwrap();
        hands.push(Hand::new_with_wildcard(hand, num_str.parse().unwrap(), 'J'))
    }

    hands.sort_by(|hand_1, hand_2| hand_1.compare(hand_2, Some('J')));
    let res = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + hand.bid * (index as u32 + 1));

    println!("res = {}", res);
}

fn get_val(c: char) -> u32 {
    match c {
        'A' => 100,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}
