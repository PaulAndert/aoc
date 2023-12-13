use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    hand_cards: Vec<u64>,
    bid: u64,
}

pub fn main() {
    let contents = fs::read_to_string("./resources/day_07").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        hands.push(convert_line(line));
    }

    // custom sort by call
    hands.sort_by(
        |a, b| if a.hand_type.cmp(&b.hand_type) == Ordering::Equal {
            for i in 0..a.hand_cards.len() {
                if a.hand_cards[i] < b.hand_cards[i] {
                    return Ordering::Greater;
                }else if a.hand_cards[i] > b.hand_cards[i] {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }else {
            a.hand_type.cmp(&b.hand_type)
        }
    );

    let mut sum: u64 = 0;
    let mut rank: u64 = 1;
    for i in (0..hands.len()).rev() {
        sum += rank * hands[i].bid;
        rank += 1;
    }

    println!("C: {}", sum);
}

fn convert_line(line: &str) -> Hand {
    let parts: Vec<&str> = line.split(" ").collect();
    let cards_char: Vec<char> = parts[0].chars().collect();
    let bid: u64 = str_to_u64(parts[1]);

    let mut pairs = HashMap::new();

    for c in 0..cards_char.len() {
        let hash_c = pairs.entry(cards_char[c]).or_insert(0);
        *hash_c += 1;
    }

    let mut hand_type: HandType = HandType::HighCard;
    for (key,value) in pairs.clone() {
        match value {
            5 => {
                hand_type = HandType::FiveOfAKind;
                break;
            },
            4 => {
                hand_type = HandType::FourOfAKind;
                break;
            },
            3 => {
                if pairs.len() == 2 {
                    hand_type = HandType::FullHouse;
                    break;
                }else {
                    hand_type = HandType::ThreeOfAKind;
                    break;
                }
            },
            2 => {
                match pairs.len() {
                    2 => {
                        hand_type = HandType::FullHouse;
                        break;
                    },
                    3 => {
                        hand_type = HandType::TwoPair;
                        break;
                    },
                    4 => {
                        hand_type = HandType::OnePair;
                        break;
                    },
                    _ => { println!("Error: unknown {} {}", key, value); }
                }
            },
            1 => {
                hand_type = HandType::HighCard;
            },
            _ => { println!("Error: unknown {} {}", key, value); }
        }
    }

    let mut hand_cards: Vec<u64> = Vec::new();
    for c in cards_char {
        hand_cards.push(match c {
            '0'..='9' => { c as u64 - '0' as u64 },
            'A' => { 14 },
            'K' => { 13 },
            'Q' => { 12 },
            'J' => { 11 },
            'T' => { 10 },
            _ => { println!("Error: unknown Card {}", c); 0 }
        })
    }

    Hand {
        hand_type,
        hand_cards,
        bid,
    }
}

fn str_to_u64(string_number: &str) -> u64 {
    let chars: Vec<char> = string_number.chars().collect();
    let mut cnt: u32 = 0;
    let mut number: u64 = 0;
    for n in (0..chars.len()).rev() {
        number += (chars[n] as u64 - '0' as u64) * u64::pow(10, cnt);
        cnt += 1;
    }
    return number;
}