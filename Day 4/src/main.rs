use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";

    // *** Part 1 ***
    let mut total_points = 0;

    // Go through each card
    for mut line in read_to_string(file_path).unwrap().lines() {
        // Remove the game number info
        line = line.split(": ").nth(1).unwrap();

        // Split into winning numbers and numbers you have
        let winning_numbers = line.split(" | ").nth(0).unwrap();
        let numbers_you_have = line.split(" | ").nth(1).unwrap();

        // Turn winning numbers in a hash set
        let mut winning_set = HashSet::<i32>::new();
        for num in winning_numbers.split(" ") {
            if num.is_empty() {
                continue;
            }
            let val: i32 = num.parse().unwrap();
            winning_set.insert(val);
        }

        // Get number of points from each card
        let mut points = 0;
        for num in numbers_you_have.split(" ") {
            if num.is_empty() {
                continue;
            }
            let val: i32 = num.parse().unwrap();
            if winning_set.contains(&val) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        total_points += points;
    }
    println!("{}", total_points);
    // *** End Part 1 ***

    // *** Part 2 ***
    // Get number of initial cards
    let init_cards = read_to_string(file_path).unwrap().lines().count();

    // Create an array of current cards
    let mut current_cards = vec![1; init_cards];

    // Go through each card
    for mut line in read_to_string(file_path).unwrap().lines() {
        // Split game number info
        let mut card_info = line.split(": ").nth(0).unwrap();

        // Get card number
        card_info = &card_info[4..].trim();
        let card_number: usize = card_info.parse().unwrap();

        // Get number of cards won
        let mut curr_cards_won = 0;
        line = line.split(": ").nth(1).unwrap();

        // Split into winning numbers and numbers you have
        let winning_numbers = line.split(" | ").nth(0).unwrap();
        let numbers_you_have = line.split(" | ").nth(1).unwrap();

        // Turn winning numbers in a hash set
        let mut winning_set = HashSet::<i32>::new();
        for num in winning_numbers.split(" ") {
            if num.is_empty() {
                continue;
            }
            let val: i32 = num.parse().unwrap();
            winning_set.insert(val);
        }

        // Get number of cards won
        for num in numbers_you_have.split(" ") {
            if num.is_empty() {
                continue;
            }
            let val: i32 = num.parse().unwrap();
            if winning_set.contains(&val) {
                curr_cards_won += 1;
            }
        }

        // Win new cards
        for i in 0..curr_cards_won {
            current_cards[i + card_number] += current_cards[card_number - 1];
        }
    }
    println!("{}", current_cards.iter().sum::<usize>());
    // *** End Part 2 ***
}
