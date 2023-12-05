use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day(challenge: u8, data: &str) -> u32 {
    match challenge {
        1 => challenge_1(data),
        2 => challenge_2(data),
        _ => 1_u32,
    }
}

fn get_seeds(line: &str) -> Vec<u32> {
    let mut seeds: Vec<u32> = Vec::new();

    let line_split = line.split(":").collect::<Vec<&str>>();
    let seeds_string = line_split[1].trim().split(" ").collect::<Vec<&str>>();

    for seed in seeds_string {
        if seed != "" {
            seeds.push(seed.parse::<u32>().unwrap());
        }
    }

    println!("Found seeds {:?}", seeds);
    seeds
}

fn get_seeds_2(line: &str) -> Vec<u32> {
    let mut seeds: Vec<u32> = Vec::new();

    let line_split = line.split(":").collect::<Vec<&str>>();
    let seeds_string = line_split[1].trim().split(" ").collect::<Vec<&str>>();

    let all_seeds = seeds_string.clone();

    // Filter for non "" entries
    let seeds_string = seeds_string
        .iter()
        .filter(|&x| x != &"")
        .collect::<Vec<&&str>>();

    // Now iterate over them 2 at a time and construct the seeds

    let output_seeds: Vec<u32> = Vec::new();
    for chunk in seeds_string.chunks(2) {
        if chunk.len() == 2 {
            let range_start = chunk[0];
            let range_length = chunk[1];

            let start = range_start.parse::<u32>().unwrap();
            let length = range_length.parse::<u32>().unwrap();

            for i in start..(start + length) {
                seeds.push(i);
            }
        } else {
            println!("Chunk is not 2 long, chunk is {:?}", chunk);
        }
    }

    println!("Found seeds {:?}", seeds);
    seeds
}

fn challenge_1(data: &str) -> u32 {
    // Read in the data file.
    let _values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // use a map to get the list of all the lines as chars
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // Get the seeds from the first line
    let mut current_map: String = String::new();
    let map_regex = Regex::new(r"map").unwrap();
    let mut map = HashMap::new();
    let mut seeds: Vec<u32> = Vec::new();
    for (index, line_vec) in lines.iter().enumerate() {
        let line = line_vec.iter().collect::<String>();
        print!("Line {}: {}", index, line);
        // Some command flow to see what we are doing
        if index == 0 {
            seeds = get_seeds(&line);
        } else if line == "" {
            // This is the end of the map
            println!("Resetting map");
            current_map = "".to_string();
        } else if map_regex.is_match(&line) {
            current_map = line.split(" ").collect::<Vec<&str>>()[0].to_string();
            println!("Current map is {}", current_map);
        } else if current_map != "" {
            // Lets make the source and destination intervals from the line
            let parts: Vec<&str> = line.split(" ").collect();
            let (ds, ss, r) = (
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
                parts[2].parse::<u32>().unwrap(),
            );

            // source end
            let se = ss + (r - 1);
            let source_interval = [ss, se];
            let dest = ds;

            // Now we will add these into the map
            let current_map_entry = map.entry(current_map.clone()).or_insert(HashMap::new());
            current_map_entry.insert(source_interval, dest);
        }
    }
    println!("Map is {:?}", map);

    // Now we need to map the seeds through the various maps to get to the locations.
    let order: Vec<&str> = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    .to_vec();

    let mut locations: Vec<u32> = Vec::new();
    for seed in seeds {
        // Trace the seed through the maps
        let mut current_value = seed;
        for map_name in order.iter() {
            // Now we check to see if the seed is in any of the ranges in the map
            let current_map = map.get(*map_name).unwrap();
            let mut found = false;

            let ranges = current_map.keys();
            for range in ranges {
                if range[0] <= current_value && range[1] >= current_value {
                    // We have found the range
                    let diff = current_value - range[0];
                    current_value = current_map.get(range).unwrap() + diff;
                    found = true;
                    break;
                }
            }

            if !found {
                println!(
                    "Seed {} not found in map {} so not doing anything",
                    seed, map_name
                );
            }

            //
        }

        locations.push(current_value);
    }

    println!("Locations are {:?}", locations);

    // Minimum location
    let min_location = locations.iter().min().unwrap();
    println!("Min location is {}", min_location);

    0_32
}

fn challenge_2(data: &str) -> u32 {
    // Read in the data file.
    let _values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // use a map to get the list of all the lines as chars
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // Get the seeds from the first line
    let mut current_map: String = String::new();
    let map_regex = Regex::new(r"map").unwrap();
    let mut map = HashMap::new();
    let mut seeds: Vec<u32> = Vec::new();
    for (index, line_vec) in lines.iter().enumerate() {
        let line = line_vec.iter().collect::<String>();
        print!("Line {}: {}", index, line);
        // Some command flow to see what we are doing
        if index == 0 {
            seeds = get_seeds_2(&line);
        } else if line == "" {
            // This is the end of the map
            println!("Resetting map");
            current_map = "".to_string();
        } else if map_regex.is_match(&line) {
            current_map = line.split(" ").collect::<Vec<&str>>()[0].to_string();
            println!("Current map is {}", current_map);
        } else if current_map != "" {
            // Lets make the source and destination intervals from the line
            let parts: Vec<&str> = line.split(" ").collect();
            let (ds, ss, r) = (
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
                parts[2].parse::<u32>().unwrap(),
            );

            // source end
            let se = ss + (r - 1);
            let source_interval = [ss, se];
            let dest = ds;

            // Now we will add these into the map
            let current_map_entry = map.entry(current_map.clone()).or_insert(HashMap::new());
            current_map_entry.insert(source_interval, dest);
        }
    }
    println!("Map is {:?}", map);

    // Now we need to map the seeds through the various maps to get to the locations.
    let order: Vec<&str> = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    .to_vec();

    let mut locations: Vec<u32> = Vec::new();
    for seed in seeds {
        // Trace the seed through the maps
        let mut current_value = seed;
        for map_name in order.iter() {
            // Now we check to see if the seed is in any of the ranges in the map
            let current_map = map.get(*map_name).unwrap();
            let mut found = false;

            let ranges = current_map.keys();
            for range in ranges {
                if range[0] <= current_value && range[1] >= current_value {
                    // We have found the range
                    let diff = current_value - range[0];
                    current_value = current_map.get(range).unwrap() + diff;
                    found = true;
                    break;
                }
            }

            if !found {
                println!(
                    "Seed {} not found in map {} so not doing anything",
                    seed, map_name
                );
            }

            //
        }

        locations.push(current_value);
    }

    println!("Locations are {:?}", locations);

    // Minimum location
    let min_location = locations.iter().min().unwrap();
    println!("Min location is {}", min_location);

    0_32
}
