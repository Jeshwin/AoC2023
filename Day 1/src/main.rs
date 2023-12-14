use std::fs::read_to_string;

fn main() {
    let mut calibration_sum = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        let mut found_nums = vec![];
        let line_len = line.len();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                found_nums.push(c.to_digit(10).unwrap_or(0));
                continue;
            }
            // Handle spelled out numbers
            if i + 5 <= line_len {
                if &line[i..i + 5] == "three" {
                    found_nums.push(3);
                    continue;
                }
                if &line[i..i + 5] == "seven" {
                    found_nums.push(7);
                    continue;
                }
                if &line[i..i + 5] == "eight" {
                    found_nums.push(8);
                    continue;
                }
            }
            if i + 4 <= line_len {
                if &line[i..i + 4] == "four" {
                    found_nums.push(4);
                    continue;
                }
                if &line[i..i + 4] == "five" {
                    found_nums.push(5);
                    continue;
                }
                if &line[i..i + 4] == "nine" {
                    found_nums.push(9);
                    continue;
                }
            }
            if i + 3 <= line_len {
                if &line[i..i + 3] == "one" {
                    found_nums.push(1);
                    continue;
                }
                if &line[i..i + 3] == "two" {
                    found_nums.push(2);
                    continue;
                }
                if &line[i..i + 3] == "six" {
                    found_nums.push(6);
                    continue;
                }
            }
        }
        calibration_sum += (found_nums[0] * 10) + found_nums[found_nums.len() - 1];
    }
    println!("{}", calibration_sum);
}
