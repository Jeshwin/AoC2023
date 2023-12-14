use std::fs::read_to_string;

const FILE_PATH: &str = "input.txt";

fn main() {
    let mut schematic = vec![];

    // *** Part 1 ***
    let mut part_number_sum = 0;

    // Create schematic as 2D array of chars
    for line in read_to_string(FILE_PATH).unwrap().lines() {
        let mut schematic_row = vec![];
        for ch in line.chars() {
            schematic_row.push(ch);
        }
        schematic.push(schematic_row);
    }

    let directions: Vec<Vec<i32>> = vec![
        vec![-1, -1],
        vec![-1, 0],
        vec![-1, 1],
        vec![0, -1],
        vec![0, 1],
        vec![1, -1],
        vec![1, 0],
        vec![1, 1],
    ];
    for i in 0..schematic.len() {
        for j in 0..schematic[0].len() {
            let curr_ch = schematic[i][j];
            // Skip numeric characters and periods
            if curr_ch.is_numeric() || curr_ch == '.' {
                continue;
            }
            // Check if adjacent chars are numbers
            for d in directions.iter() {
                let ni = (i as i32 + d[0]) as usize;
                let nj = (j as i32 + d[1]) as usize;
                let adj_ch = schematic[ni][nj];
                if adj_ch.is_numeric() {
                    let mut adj_number = 0;
                    // Case 1a: Last digit of a three digit number
                    if nj >= 2
                        && schematic[ni][nj - 2].is_numeric()
                        && schematic[ni][nj - 1].is_numeric()
                    {
                        adj_number += schematic[ni][nj - 2].to_digit(10).unwrap() * 100;
                        adj_number += schematic[ni][nj - 1].to_digit(10).unwrap() * 10;
                        adj_number += schematic[ni][nj].to_digit(10).unwrap();
                        // Overwrite used numbers
                        schematic[ni][nj - 2] = '.';
                        schematic[ni][nj - 1] = '.';
                    }
                    // Case 1b: Second digit of a three digit number
                    else if nj >= 1
                        && nj <= schematic[0].len() - 2
                        && schematic[ni][nj - 1].is_numeric()
                        && schematic[ni][nj + 1].is_numeric()
                    {
                        adj_number += schematic[ni][nj - 1].to_digit(10).unwrap() * 100;
                        adj_number += schematic[ni][nj].to_digit(10).unwrap() * 10;
                        adj_number += schematic[ni][nj + 1].to_digit(10).unwrap();
                        // Overwrite used numbers
                        schematic[ni][nj - 1] = '.';
                        schematic[ni][nj + 1] = '.';
                    }
                    // Case 1c: First digit of a three digit number
                    else if nj <= schematic[0].len() - 3
                        && schematic[ni][nj + 1].is_numeric()
                        && schematic[ni][nj + 2].is_numeric()
                    {
                        adj_number += schematic[ni][nj].to_digit(10).unwrap() * 100;
                        adj_number += schematic[ni][nj + 1].to_digit(10).unwrap() * 10;
                        adj_number += schematic[ni][nj + 2].to_digit(10).unwrap();
                        // Overwrite used numbers
                        schematic[ni][nj + 1] = '.';
                        schematic[ni][nj + 2] = '.';
                    }
                    // Case 2a: Second digit of a two digit number
                    else if nj >= 1 && schematic[ni][nj - 1].is_numeric() {
                        adj_number += schematic[ni][nj - 1].to_digit(10).unwrap() * 10;
                        adj_number += schematic[ni][nj].to_digit(10).unwrap();
                        // Overwrite used numbers
                        schematic[ni][nj - 1] = '.';
                    }
                    // Case 2b: First digit of a two digit number
                    else if nj <= schematic[0].len() - 2 && schematic[ni][nj + 1].is_numeric() {
                        adj_number += schematic[ni][nj].to_digit(10).unwrap() * 10;
                        adj_number += schematic[ni][nj + 1].to_digit(10).unwrap();
                        // Overwrite used numbers
                        schematic[ni][nj + 1] = '.'
                    }
                    // Case 3: Single digit number
                    else {
                        adj_number += schematic[ni][nj].to_digit(10).unwrap();
                    }
                    schematic[ni][nj] = '.';
                    part_number_sum += adj_number;
                }
            }
        }
    }
    println!("Part Number Sum: {}", part_number_sum);
    // *** End Part 1 ***

    // *** Part 2 ***
    let mut gear_sum = 0;

    // Regenerate schematic
    for line in read_to_string(FILE_PATH).unwrap().lines() {
        let mut schematic_row = vec![];
        for ch in line.chars() {
            schematic_row.push(ch);
        }
        schematic.push(schematic_row);
    }

    let directions: Vec<Vec<i32>> = vec![
        vec![-1, -1],
        vec![-1, 0],
        vec![-1, 1],
        vec![0, -1],
        vec![0, 1],
        vec![1, -1],
        vec![1, 0],
        vec![1, 1],
    ];
    for i in 0..schematic.len() {
        for j in 0..schematic[0].len() {
            let curr_ch = schematic[i][j];
            // Skip numeric characters and periods
            if curr_ch != '*' {
                continue;
            }
            // Get all adjacent numbers
            let mut adj_numbers = vec![];
            for d in directions.iter() {
                let ni = (i as i32 + d[0]) as usize;
                let nj = (j as i32 + d[1]) as usize;
                let adj_ch = schematic[ni][nj];
                if !adj_ch.is_numeric() {
                    continue;
                }
                let mut adj_number = 0;
                // Case 1a: Last digit of a three digit number
                if nj >= 2
                    && schematic[ni][nj - 2].is_numeric()
                    && schematic[ni][nj - 1].is_numeric()
                {
                    adj_number += schematic[ni][nj - 2].to_digit(10).unwrap() * 100;
                    adj_number += schematic[ni][nj - 1].to_digit(10).unwrap() * 10;
                    adj_number += schematic[ni][nj].to_digit(10).unwrap();
                    // Overwrite used numbers
                    schematic[ni][nj - 2] = '.';
                    schematic[ni][nj - 1] = '.';
                }
                // Case 1b: Second digit of a three digit number
                else if nj >= 1
                    && nj <= schematic[0].len() - 2
                    && schematic[ni][nj - 1].is_numeric()
                    && schematic[ni][nj + 1].is_numeric()
                {
                    adj_number += schematic[ni][nj - 1].to_digit(10).unwrap() * 100;
                    adj_number += schematic[ni][nj].to_digit(10).unwrap() * 10;
                    adj_number += schematic[ni][nj + 1].to_digit(10).unwrap();
                    // Overwrite used numbers
                    schematic[ni][nj - 1] = '.';
                    schematic[ni][nj + 1] = '.';
                }
                // Case 1c: First digit of a three digit number
                else if nj <= schematic[0].len() - 3
                    && schematic[ni][nj + 1].is_numeric()
                    && schematic[ni][nj + 2].is_numeric()
                {
                    adj_number += schematic[ni][nj].to_digit(10).unwrap() * 100;
                    adj_number += schematic[ni][nj + 1].to_digit(10).unwrap() * 10;
                    adj_number += schematic[ni][nj + 2].to_digit(10).unwrap();
                    // Overwrite used numbers
                    schematic[ni][nj + 1] = '.';
                    schematic[ni][nj + 2] = '.';
                }
                // Case 2a: Second digit of a two digit number
                else if nj >= 1 && schematic[ni][nj - 1].is_numeric() {
                    adj_number += schematic[ni][nj - 1].to_digit(10).unwrap() * 10;
                    adj_number += schematic[ni][nj].to_digit(10).unwrap();
                    // Overwrite used numbers
                    schematic[ni][nj - 1] = '.';
                }
                // Case 2b: First digit of a two digit number
                else if nj <= schematic[0].len() - 2 && schematic[ni][nj + 1].is_numeric() {
                    adj_number += schematic[ni][nj].to_digit(10).unwrap() * 10;
                    adj_number += schematic[ni][nj + 1].to_digit(10).unwrap();
                    // Overwrite used numbers
                    schematic[ni][nj + 1] = '.'
                }
                // Case 3: Single digit number
                else {
                    adj_number += schematic[ni][nj].to_digit(10).unwrap();
                }
                schematic[ni][nj] = '.';
                adj_numbers.push(adj_number);
            }
            if adj_numbers.len() == 2 {
                gear_sum += adj_numbers[0] * adj_numbers[1];
            }
        }
    }
    println!("Gear Sum: {}", gear_sum);
    // *** End Part 2 ***
}
