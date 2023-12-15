use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::read_to_string;

enum Mappings {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation,
    Empty,
}

fn main() {
    let file_path = "input.txt";
    let file_data = read_to_string(file_path).unwrap();

    // Get all the seed pairs
    let seed_pair_line = file_data.lines().nth(0).unwrap();
    let seed_pairs: Vec<u64> = seed_pair_line[7..]
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    // Get all seeds
    let mut seeds = Vec::new();
    for i in 0..seed_pairs.len() / 2 {
        let seed_start = seed_pairs[2 * i];
        let seed_range = seed_pairs[(2 * i) + 1];
        for s in 0..seed_range {
            seeds.push(seed_start + s);
        }
    }

    // Store almanac mappings as 2D arrays
    let mut seed_soil: Vec<Vec<u64>> = Vec::new();
    let mut soil_fertilizer: Vec<Vec<u64>> = Vec::new();
    let mut fertilizer_water: Vec<Vec<u64>> = Vec::new();
    let mut water_light: Vec<Vec<u64>> = Vec::new();
    let mut light_temp: Vec<Vec<u64>> = Vec::new();
    let mut temp_humidity: Vec<Vec<u64>> = Vec::new();
    let mut humidity_location: Vec<Vec<u64>> = Vec::new();
    let mut curr_state = Mappings::Empty;
    for line in file_data.lines() {
        // Skip empty lines
        if line.is_empty() {
            curr_state = Mappings::Empty;
            continue;
        }
        // Skip seed line
        if &line[..5] == "seeds" {
            continue;
        }
        // Find out which mapping we are on
        match line {
            "seed-to-soil map:" => {
                curr_state = Mappings::SeedToSoil;
                continue;
            }
            "soil-to-fertilizer map:" => {
                curr_state = Mappings::SoilToFertilizer;
                continue;
            }
            "fertilizer-to-water map:" => {
                curr_state = Mappings::FertilizerToWater;
                continue;
            }
            "water-to-light map:" => {
                curr_state = Mappings::WaterToLight;
                continue;
            }
            "light-to-temperature map:" => {
                curr_state = Mappings::LightToTemp;
                continue;
            }
            "temperature-to-humidity map:" => {
                curr_state = Mappings::TempToHumidity;
                continue;
            }
            "humidity-to-location map:" => {
                curr_state = Mappings::HumidityToLocation;
                continue;
            }
            _ => {}
        }
        // Get mapping value
        let mapping_value: Vec<u64> = line.split(" ").map(|s| s.parse().unwrap()).collect();

        // Put mapping value in correct mapping
        match curr_state {
            Mappings::SeedToSoil => seed_soil.push(mapping_value),
            Mappings::SoilToFertilizer => soil_fertilizer.push(mapping_value),
            Mappings::FertilizerToWater => fertilizer_water.push(mapping_value),
            Mappings::WaterToLight => water_light.push(mapping_value),
            Mappings::LightToTemp => light_temp.push(mapping_value),
            Mappings::TempToHumidity => temp_humidity.push(mapping_value),
            Mappings::HumidityToLocation => humidity_location.push(mapping_value),
            Mappings::Empty => {}
        }
    }

    // Create a loading bar
    let pb = ProgressBar::new(seeds.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Get location for each seed
    let min_location = seeds
        .par_iter() // Use par_iter() for parallel iteration
        .map(|&seed| {
            let mut result = calculate_next_value(seed, &seed_soil);
            result = calculate_next_value(result, &soil_fertilizer);
            result = calculate_next_value(result, &fertilizer_water);
            result = calculate_next_value(result, &water_light);
            result = calculate_next_value(result, &light_temp);
            result = calculate_next_value(result, &temp_humidity);
            result = calculate_next_value(result, &humidity_location);
            pb.inc(1); // Increment progress bar
            result
        })
        .reduce(|| std::u64::MAX, |min, val| std::cmp::min(min, val));

    pb.finish(); // Finish the progress bar

    println!("Min Location: {}", min_location);
}

fn calculate_next_value(curr_value: u64, mapping: &Vec<Vec<u64>>) -> u64 {
    for entry in mapping.into_iter() {
        if curr_value >= entry[1] && curr_value < entry[1] + entry[2] {
            return curr_value - entry[1] + entry[0];
        }
    }
    curr_value
}
