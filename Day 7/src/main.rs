use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

fn main() {
    let file_path = "input.txt";
    let file_data = read_to_string(file_path).unwrap();

    // *** Part 1 ***
    let mut total_winninngs = 0;
    // Store hands and bids into array
    let mut hands_and_bids: Vec<Vec<&str>> = Vec::new();
    for line in file_data.lines() {
        hands_and_bids.push(line.split_whitespace().collect());
    }
    // Sort array by decreasing hand strength
    hands_and_bids.sort_by(|a, b| compare_hands_part_one(a[0], b[0]));
    for i in 0..hands_and_bids.len() {
        total_winninngs += (i + 1) * hands_and_bids[i][1].parse::<usize>().unwrap();
    }
    println!("Part 1 -> {}", total_winninngs);
    // *** End Part 1 ***

    // *** Part 2 ***
    total_winninngs = 0;
    // Use the new Joker
    hands_and_bids.sort_by(|a, b| compare_hands_part_two(a[0], b[0]));
    for i in 0..hands_and_bids.len() {
        total_winninngs += (i + 1) * hands_and_bids[i][1].parse::<usize>().unwrap();
    }
    println!("Part 1 -> {}", total_winninngs);
    // *** End Part 2 ***
}

fn compare_hands_part_one(lhs: &str, rhs: &str) -> Ordering {
    // Get types of each hand
    let lhs_type = hand_type_part_one(lhs).unwrap() as u32;
    let rhs_type = hand_type_part_one(rhs).unwrap() as u32;

    if lhs_type > rhs_type {
        return Ordering::Greater;
    } else if rhs_type > lhs_type {
        return Ordering::Less;
    }
    // Hand types are the same; compare cards in order
    let cards = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    for i in 0..5 {
        let lhs_card_val = cards
            .iter()
            .position(|&c| c == lhs.chars().nth(i).unwrap())
            .unwrap();
        let rhs_card_val: usize = cards
            .iter()
            .position(|&c| c == rhs.chars().nth(i).unwrap())
            .unwrap();
        if lhs_card_val > rhs_card_val {
            return Ordering::Greater;
        } else if lhs_card_val < rhs_card_val {
            return Ordering::Less;
        }
    }
    // The hands must be the same
    Ordering::Equal
}

fn hand_type_part_one(hand: &str) -> Option<HandType> {
    let mut cards = HashMap::<char, i32>::new();
    hand.chars().for_each(|c| *cards.entry(c).or_insert(0) += 1);
    if cards.len() == 1 {
        Some(HandType::FiveKind)
    } else if cards.len() == 4 {
        Some(HandType::OnePair)
    } else if cards.len() == 5 {
        Some(HandType::HighCard)
    } else if cards.len() == 2 {
        if cards.into_values().any(|x| x == 4) {
            Some(HandType::FourKind)
        } else {
            Some(HandType::FullHouse)
        }
    } else if cards.len() == 3 {
        if cards.into_values().any(|x| x == 3) {
            Some(HandType::ThreeKind)
        } else {
            Some(HandType::TwoPair)
        }
    } else {
        None
    }
}

fn compare_hands_part_two(lhs: &str, rhs: &str) -> Ordering {
    // Get types of each hand
    let lhs_type = hand_type_part_two(lhs).unwrap() as u32;
    let rhs_type = hand_type_part_two(rhs).unwrap() as u32;
    if lhs_type > rhs_type {
        return Ordering::Greater;
    } else if rhs_type > lhs_type {
        return Ordering::Less;
    }
    // Hand types are the same; compare cards in order
    let cards = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    for i in 0..5 {
        let lhs_card_val = cards
            .iter()
            .position(|&c| c == lhs.chars().nth(i).unwrap())
            .unwrap();
        let rhs_card_val: usize = cards
            .iter()
            .position(|&c| c == rhs.chars().nth(i).unwrap())
            .unwrap();
        if lhs_card_val > rhs_card_val {
            return Ordering::Greater;
        } else if lhs_card_val < rhs_card_val {
            return Ordering::Less;
        }
    }
    // The hands must be the same
    Ordering::Equal
}

fn hand_type_part_two(hand: &str) -> Option<HandType> {
    let mut cards = HashMap::<char, i32>::new();
    let mut num_jokers = 0;
    for card in hand.chars() {
        if card == 'J' {
            num_jokers += 1;
            continue;
        }
        *cards.entry(card).or_insert(0) += 1;
    }
    if num_jokers >= 4 || cards.len() == 1 {
        return Some(HandType::FiveKind);
    } else if num_jokers == 3 {
        return Some(HandType::FourKind);
    } else if num_jokers == 2 {
        // Case 1: 2 cards match
        if cards.into_values().any(|x| x == 2) {
            return Some(HandType::FourKind);
        // Case 2: All cards differ
        } else {
            return Some(HandType::ThreeKind);
        }
    } else if num_jokers == 1 {
        // Case 1: 2 types of cards
        if cards.len() == 2 {
            // Case 1a: 3 cards match
            if cards.into_values().any(|x| x == 3) {
                return Some(HandType::FourKind);
            // Case 2: 2 pairs
            } else {
                return Some(HandType::FullHouse);
            }
        } else if cards.len() == 3 {
            return Some(HandType::ThreeKind);
        } else {
            return Some(HandType::OnePair);
        }
    } else {
        return hand_type_part_one(hand);
    }
}
