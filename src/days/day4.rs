use core::cmp::min;

use ndarray::s;
use ndarray::Array2;
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

fn get_card_number(line: &str) -> u32 {
    let split_line: Vec<&str> = line.split(':').collect();

    // Now get the number
    let card_number: Vec<&str> = split_line[0].split(' ').collect();

    card_number[card_number.len() - 1].parse::<u32>().unwrap()
}

fn get_winning_numbers(line: &str) -> HashSet<u32> {
    let split_line: Vec<&str> = line.split(':').collect();

    // Now get the number
    let winning_numbers: Vec<&str> = split_line[1].split('|').collect();

    let mut numbers: Vec<u32> = Vec::new();

    // The numbers are space separated so we need to split on the space
    // and then parse the string into a vector of u32.

    let winning_numbers = winning_numbers[0].split(' ').collect::<Vec<&str>>();

    for number in winning_numbers {
        if number != "" {
            numbers.push(number.parse::<u32>().unwrap());
        }
    }

    let numbers_hash: HashSet<u32> = numbers.into_iter().collect();
    numbers_hash
}

fn get_our_numbers(line: &str) -> HashSet<u32> {
    let split_line: Vec<&str> = line.split(':').collect();

    // Now get the number
    let winning_numbers: Vec<&str> = split_line[1].split('|').collect();

    let mut numbers: Vec<u32> = Vec::new();

    // The numbers are space separated so we need to split on the space
    // and then parse the string into a vector of u32.

    let winning_numbers = winning_numbers[1].split(' ').collect::<Vec<&str>>();

    for number in winning_numbers {
        if number != "" {
            numbers.push(number.parse::<u32>().unwrap());
        }
    }

    let numbers_hash: HashSet<u32> = numbers.into_iter().collect();
    numbers_hash
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

    // We're going to do the same thing as before and split the
    // line on the semicolon and then split on the | to get two vectors,
    // turn those vectors into sets, and the find the value for each one.

    let mut total = 0;

    for line in lines.iter() {
        let card_number = get_card_number(&line.iter().collect::<String>());
        let winning_numbers = get_winning_numbers(&line.iter().collect::<String>());
        let our_numbers = get_our_numbers(&line.iter().collect::<String>());

        let intersection: Vec<u32> = our_numbers
            .intersection(&winning_numbers)
            .cloned()
            .collect();
        let mut matches = intersection.len();

        println!(
            "Card number: {}, winning numbers: {:?}, our numbers: {:?}, matches: {}",
            card_number, winning_numbers, our_numbers, matches
        );

        if matches > 0 {
            total += 2_u32.pow(matches as u32 - 1_u32);
        }
    }

    println!("Day 4 challenge 1: {}", total);
    total
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

    // We're going to do the same thing as before and split the
    // line on the semicolon and then split on the | to get two vectors,
    // turn those vectors into sets, and the find the value for each one.

    let mut total = 0;

    // We're going to keep track of the multipliers for each card number
    // with an integer
    let mut multipliers: HashMap<u32, u32> = HashMap::new();

    // We're also goign to keep track of how many points you get
    // for each card number, then at the end we'll multiply the
    // points by the multiplier.
    let mut points: HashMap<u32, u32> = HashMap::new();

    let mut highest_card_number = 0;

    for line in lines.iter() {
        let card_number = get_card_number(&line.iter().collect::<String>());
        let winning_numbers = get_winning_numbers(&line.iter().collect::<String>());
        let our_numbers = get_our_numbers(&line.iter().collect::<String>());

        // Add 1 or increment the multiplier for the card number
        if multipliers.contains_key(&card_number) {
            let current_multiplier = multipliers.entry(card_number).or_insert(1);
            // *current_multiplier += 1;
        } else {
            multipliers.insert(card_number, 1);
        }

        let intersection: Vec<u32> = our_numbers
            .intersection(&winning_numbers)
            .cloned()
            .collect();
        let mut matches = intersection.len();

        // println!(
        //     "Card number: {}, winning numbers: {:?}, our numbers: {:?}, matches: {}",
        //     card_number, winning_numbers, our_numbers, matches
        // );

        if matches > 0 {
            points.insert(card_number, 2_u32.pow(matches as u32 - 1_u32));

            // Get the current multiplier for this card number
            let current_card_multiplier = *multipliers.entry(card_number).or_insert(1);

            // Now we're going to add the multiplier to the card number starting with
            // the next card number after the current one

            println!("Found {} matches so adding multipliers", matches);
            for nmatch in 0..matches {
                let next_card_number = card_number + nmatch as u32 + 1_u32;

                // Now add current_card_multiplier to the next card number
                let next_card_multiplier = multipliers.entry(next_card_number).or_insert(1);
                *next_card_multiplier += current_card_multiplier;

                println!("{:?}", multipliers);
            }
        }

        // Now we need to find the highest card number
        if card_number > highest_card_number {
            highest_card_number = card_number;
        }
    }

    print!("Multipliers: {:?}", multipliers);
    print!("Points: {:?}", points);

    for card in 1..highest_card_number + 1 {
        let multiplier = multipliers.entry(card);

        total += multiplier;
    }

    // Now we iterate over the points and multiply them by the multiplier
    // for (card_number, point) in points.iter() {
    //     let multiplier = multipliers.entry(*card_number).or_insert(1);
    //     total += *multiplier;
    // }

    println!("Day 4 challenge 1: {}", total);
    total
}
