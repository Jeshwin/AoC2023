use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";
    let file_data = read_to_string(file_path).unwrap();

    // *** Part 1 ***
    let mut part_one = 1;
    // Read file
    let time_line = &file_data.lines().nth(0).unwrap()[5..];
    let distance_line = &file_data.lines().nth(1).unwrap()[9..];

    // Get times and distances
    let times: Vec<usize> = time_line.split_whitespace().flat_map(str::parse).collect();
    let distances: Vec<usize> = distance_line
        .split_whitespace()
        .flat_map(str::parse)
        .collect();

    for i in 0..times.len() {
        let mut poss_distances = vec![0; times[i] + 1];
        for j in 0..poss_distances.len() {
            poss_distances[j] = j * (times[i] - j);
        }
        let distance_to_beat = distances[i];
        let mut min_hold = 0;
        let mut max_hold = 0;
        for j in 0..poss_distances.len() {
            if poss_distances[j] > distance_to_beat {
                min_hold = j;
                break;
            }
        }
        for j in (0..poss_distances.len()).rev() {
            if poss_distances[j] > distance_to_beat {
                max_hold = j;
                break;
            }
        }
        part_one *= max_hold - min_hold + 1;
    }
    println!("Part 1 -> {}", part_one);
    // *** End Part 1 ***

    // *** Part 2 ***
    let mut part_two = 1;

    // Get the time and distance
    let big_time = file_data
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let big_distance = file_data
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    for pressed_time in 0..big_time + 1 {
        let distance = pressed_time * (big_time - pressed_time);
        if distance > big_distance {
            part_two = big_time - 2 * pressed_time + 1;
            break;
        }
    }

    println!("Part 2 -> {}", part_two);
    // *** End Part 2 ***
}
