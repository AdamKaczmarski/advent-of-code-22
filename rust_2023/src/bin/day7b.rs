use anyhow::Result;
use itertools::Itertools;
use std::{cmp::Ordering, fs, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn type_value(&self) -> u8 {
        return match self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        };
    }
    fn three_of_kind_or_two_pair(card_counts: &Vec<(usize, char)>, wildcard_count:usize) -> HandType {
        for (count, _card) in card_counts {
            if *count+wildcard_count == 3 {
                return HandType::ThreeOfAKind;
            }
        }
        return Self::TwoPair;
    }
    fn four_of_kind_or_full_house(card_counts: &Vec<(usize, char)>, wildcard_count:usize) -> HandType {
        for (count, _card) in card_counts {
            if *count+wildcard_count == 4 {
                return Self::FourOfAKind;
            }
        }
        return Self::FullHouse;
    }
}

impl FromStr for HandType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let card_counts = s.chars().sorted().dedup_with_count().collect_vec();
        let has_wildcard = s.contains('J');
        let length;
        let mut wildcard_count = 0;
        if has_wildcard {
            length = card_counts.len() - 1;
            //I could do map, but most likely just doing
            //a for loop over max 5 elements is faster.
            for (x, c) in &card_counts {
                if c == &'J' {
                    wildcard_count = x.clone();
                }
            }
        } else {
            length = card_counts.len()
        }
        let htype = match length {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => HandType::three_of_kind_or_two_pair(&card_counts, wildcard_count),
            2 => HandType::four_of_kind_or_full_house(&card_counts, wildcard_count),
            1 => HandType::FiveOfAKind,
            0 => HandType::FiveOfAKind,
            _ => panic!("Couldnt get hand type from length={}, s={:?}", length, s),
        };
        return Ok(htype);
    }
}

#[derive(Clone, Debug)]
struct HandBid {
    hand: String,
    bid: usize,
    hand_type: HandType,
}

impl FromStr for HandBid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data = s.split_whitespace().collect_vec();
        let hand_type = HandType::from_str(data.get(0).unwrap()).unwrap();
        let hand: String = data.get(0).unwrap().to_string();
        return Ok(Self {
            hand,
            bid: s[6..].parse().unwrap(),
            hand_type,
        });
    }
}

fn get_card_value(card: &char) -> u8 {
    return match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("bad card here {:?}", card),
    };
}

fn sort_by_type(h1: &HandType, h2: &HandType) -> Ordering {
    if !h1.eq(h2) {
        if h1.type_value() > h2.type_value() {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn sort_by_cards(h1: &String, h2: &String) -> Ordering {
    let iter = h1.chars().zip(h2.chars());
    for (c1, c2) in iter {
        if c1 == c2 {
            continue;
        } else if get_card_value(&c1) > get_card_value(&c2) {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn main() -> Result<()> {
    let sum: usize = fs::read_to_string("./inputs/day7.prod")?
        .lines()
        .map(|hand_bid| {
            let hand_bid = HandBid::from_str(hand_bid).unwrap();
            return hand_bid;
        })
        .sorted_by(|h1: &HandBid, h2: &HandBid| sort_by_cards(&h1.hand, &h2.hand))
        .sorted_by(|h1: &HandBid, h2: &HandBid| sort_by_type(&h1.hand_type, &h2.hand_type))
        .enumerate()
        .map(|(idx, hand)| {
            return hand.bid * (idx + 1 as usize);
        })
        .sum();
    println!("{}", sum);

    Ok(())
}
