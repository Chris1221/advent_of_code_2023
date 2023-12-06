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

fn challenge_1(data: &str) -> u32 {
    // Read in the data file.
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // use a map to get the list of all the lines as chars
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let times: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    assert!(times.len() == distances.len());

    let mut totals = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        // For a time t, we have t options for evalauting whether
        // we win.
        let mut num_ways_to_win = 0;
        for t in 0..=*time {
            let distance_travelled = t * (*time - t);
            if distance_travelled > *distance {
                num_ways_to_win += 1;
            }
        }
        totals.push(num_ways_to_win);
    }

    let total: u32 = totals.iter().product();
    println!("Total is {}", total);

    0_u32
}

fn challenge_2(data: &str) -> u32 {
    // Read in the data file.
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // use a map to get the list of all the lines as chars
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let times: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut time_string = String::new();
    let mut distance_string = String::new();

    for time in times {
        time_string.push_str(&time.to_string());
    }

    for distance in distances {
        distance_string.push_str(&distance.to_string());
    }

    println!("Time string is {}", time_string);
    println!("Distance string is {}", distance_string);

    let time = time_string.parse::<u64>().unwrap();
    let distance = distance_string.parse::<u64>().unwrap();

    let mut num_ways_to_win = 0;
    for t in 0..=time {
        let distance_travelled = t * (time - t);
        if distance_travelled > distance {
            num_ways_to_win += 1;
        }
    }

    println!("Num ways to win is {}", num_ways_to_win);

    0_u32
}
