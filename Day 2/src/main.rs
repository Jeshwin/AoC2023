use std::cmp;
use std::fs::read_to_string;

// const max_reds = 12;
// const max_greens = 13;
// const max_blues = 14;

fn main() {
    // let mut id_sum = 0;
    let mut power_sum = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        // let mut parts = line.split(" ").collect::<Vec<&str>>();
        // Get ID of the game
        // let mut game_id = *(parts.get(1).unwrap());
        // game_id = &game_id[..game_id.len() - 1];
        // let line_id = game_id.parse().unwrap();
        // parts = parts[2..].to_vec();

        let mut curr_reds = 0;
        let mut curr_greens = 0;
        let mut curr_blues = 0;
        let mut max_reds = 0;
        let mut max_greens = 0;
        let mut max_blues = 0;
        let mut curr_val = 0;
        for part in line.split(" ") {
            if part == "Game" || part.chars().last().unwrap() == ':' {
                continue;
            }
            if let Ok(part_val) = part.parse::<u32>() {
                curr_val = part_val;
            } else {
                match &part[..part.len() - 1] {
                    "red" => curr_reds += curr_val,
                    "green" => curr_greens += curr_val,
                    "blue" => curr_blues += curr_val,
                    _ => {}
                }
                // Last element doesn't end in a comma or semicolor
                match part {
                    "red" => curr_reds += curr_val,
                    "green" => curr_greens += curr_val,
                    "blue" => curr_blues += curr_val,
                    _ => {}
                }
                if part.chars().last().unwrap() == ';'
                    || part.chars().last().unwrap() == 'd'
                    || part.chars().last().unwrap() == 'n'
                    || part.chars().last().unwrap() == 'e'
                {
                    max_reds = cmp::max(max_reds, curr_reds);
                    max_greens = cmp::max(max_greens, curr_greens);
                    max_blues = cmp::max(max_blues, curr_blues);
                    curr_reds = 0;
                    curr_greens = 0;
                    curr_blues = 0;
                }
            }
        }
        let game_power = max_reds * max_greens * max_blues;
        // println!(
        //     "Game Power: {} * {} * {} = {}",
        //     max_reds, max_greens, max_blues, game_power
        // );
        power_sum += game_power;
    }
    println!("{}", power_sum);
}
