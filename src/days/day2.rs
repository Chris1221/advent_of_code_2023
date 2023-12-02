use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day(challenge: u8, data: &str) -> u32 {
    match challenge {
        1 => challenge_1(data),
        2 => challenge_2(data),
        _ => 1 as u32,
    }
}

fn get_game_index(line: &str) -> u32 {
    let split_line: Vec<&str> = line.split(":").collect();

    // Now get the number
    let game_index: Vec<&str> = split_line[0].split(" ").collect();
    let number = game_index[1].parse::<u32>().unwrap();

    return number;
}
/// The input looks like this
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// So we need to split on the semicolon and then split on the comma.
fn assemble_draws(line: &str) -> HashMap<u32, HashMap<&str, u32>> {
    // Create a dictionary to hold the draws.
    let mut draws: HashMap<u32, HashMap<&str, u32>> = HashMap::new();

    // Split the line on the semicolon to get the "games"
    let split_line: Vec<&str> = line.split(";").collect();

    // For each of the games, we need to split by comma
    for (idx, game) in split_line.iter().enumerate() {
        let split_game: Vec<&str> = game.split(",").collect();

        let mut game_draws: HashMap<&str, u32> = HashMap::new();

        for draw in split_game {
            let trimmed: &str = draw.trim();

            let color = trimmed.split(" ").collect::<Vec<&str>>()[1].trim();
            let number = trimmed.split(" ").collect::<Vec<&str>>()[0]
                .trim()
                .parse::<u32>()
                .unwrap();

            // Now we need to add the color and number to the hashmap
            // if the color is already in the hashmap then we need to
            // add the number to the existing value.
            game_draws.insert(color, number);
        }

        // Now we need to add the game draws to the draws hashmap.
        // let game_id = format!("game_{}", idx);
        draws.insert(idx as u32, game_draws);
    }

    return draws;
}

/// Make a hashmap with the totals for each colour.
/// The 'static lifetime is required because the hashmap
/// is returned from the function and the compiler needs
/// to know how long the hashmap will live (forever!)
fn make_totals(red: u32, green: u32, blue: u32) -> HashMap<&'static str, u32> {
    let mut totals: HashMap<&str, u32> = HashMap::new();

    totals.insert("red", red);
    totals.insert("green", green);
    totals.insert("blue", blue);

    return totals;
}

fn compare_to_totals(totals: &HashMap<&str, u32>, draws: HashMap<u32, HashMap<&str, u32>>) -> bool {
    // Loop over the draws hashmap and compare the values to the totals
    for (game, draw) in draws {
        for (color, number) in draw {
            // if cfg!(debug_assertions) {
            //     println!("Game: {}", game);
            //     println!("Color: {}", color);
            //     println!("Number: {}", number);
            // }

            if number > *totals.get(color).unwrap() {
                return false;
            }
        }
    }

    return true;
}

fn challenge_1(data: &str) -> u32 {
    // So the first step is to split the string into
    // the "game" portion and the "draws" portion.
    let mut values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // TIL there are no named parmaeters in Rust.
    let totals: HashMap<&str, u32> = make_totals(12, 13, 14);

    // Read the file line by line into a vector of strings.
    for line in reader.lines() {
        values.push(line.unwrap());
    }

    // Alright now we iterate over the values
    // and process them.
    let mut total = 0;

    for line in values {
        // Split the line on ":" to get the game
        // index and the draws.
        let split_line: Vec<&str> = line.split(":").collect();

        let index: u32 = get_game_index(&line);
        let draws: HashMap<u32, HashMap<&str, u32>> = assemble_draws(split_line[1]);

        if cfg!(debug_assertions) {
            println!("Line: {}", line);
            println!("Index: {}", index);
            println!("Draws: {:?}", draws);
        }

        if compare_to_totals(&totals, draws) {
            println!("Adding index: {}", index);
            total += index;
        }
    }

    print!("Day 2 challenge 1: {total}", total = total);

    return total;
}

/// Find the power of the cubes
fn find_power_of_cubes(draws: HashMap<u32, HashMap<&str, u32>>) -> u32 {
    let mut power = 0;

    // Create a hashmap to hold the maximum value
    let mut maxes: HashMap<&str, u32> = HashMap::new();

    // Add 0 values for "red", "blue", "green"
    maxes.insert("red", 0);
    maxes.insert("blue", 0);
    maxes.insert("green", 0);

    // Loop over the draws hashmap and find the maximum
    // value in each game.
    for (game, draw) in draws {
        for (color, number) in draw {
            if maxes.get(color).unwrap() < &number {
                maxes.insert(color, number);
            }
        }
    }

    // Multiply all the values in the maxes hashmap
    let values: Vec<u32> = maxes.values().cloned().collect();
    let power: u32 = values.iter().product();

    return power;
}

fn challenge_2(data: &str) -> u32 {
    // So the first step is to split the string into
    // the "game" portion and the "draws" portion.
    let mut values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // TIL there are no named parmaeters in Rust.
    let totals: HashMap<&str, u32> = make_totals(12, 13, 14);

    // Read the file line by line into a vector of strings.
    for line in reader.lines() {
        values.push(line.unwrap());
    }

    // Alright now we iterate over the values
    // and process them.
    let mut total = 0;

    for line in values {
        // Split the line on ":" to get the game
        // index and the draws.
        let split_line: Vec<&str> = line.split(":").collect();

        let index: u32 = get_game_index(&line);
        let draws: HashMap<u32, HashMap<&str, u32>> = assemble_draws(split_line[1]);

        if cfg!(debug_assertions) {
            println!("Line: {}", line);
            println!("Index: {}", index);
            println!("Draws: {:?}", draws);
        }

        let power = find_power_of_cubes(draws);

        if cfg!(debug_assertions) {
            println!("Power: {}", power);
        }
        total += power;
    }

    print!("Day 2 challenge 1: {total}", total = total);

    return total;
}
