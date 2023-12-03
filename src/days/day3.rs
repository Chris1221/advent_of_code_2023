use core::cmp::max;
use core::cmp::min;
use core::num;
use ndarray::s;
use ndarray::Array2;
use std::collections::HashMap;
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
    let mut values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // use a map to get the list of all the lines as chars
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // Construct an ndarray of characters from the lines
    let rows = lines.len();
    let cols = lines[0].len();

    // Initialize with empty char and fill with the
    // characters from the lines
    let mut array = Array2::from_elem((rows, cols), ' ');
    for (i, line) in lines.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            array[[i, j]] = c;
        }
    }

    // println!("{:?}", array);

    // Visit each of the cells once. Construct a number from the sequential
    // values until it hits a "." or non-numeric value. Then check the indices
    // of the surrounding cells to see if they are occupied. If they are then
    // store the number

    let mut total = 0;

    for ridx in 0..rows {
        // Must be String not &str because I need it to be mutable.
        let mut number: String = String::new();

        // This will be the column index of the start of the number
        // I am using it to decide which cells to check for occupancy
        // Reset to 0 when I store or reject a number
        let mut number_start_c_idx = 0;

        for cidx in 0..cols {
            // If the char is a digit then I need to store it
            if array[[ridx, cidx]].is_ascii_digit() {
                number.push(array[[ridx, cidx]]);
                // println!("Adding digit: {}", array[[ridx, cidx]]);
                // println!("Number: {}", number);

                // If this is the first digit then I need to store the column index
                // so that I can check the surrounding cells for occupancy.
                // NOTE: I will reset this to 0 when I store or reject an umber
                if number_start_c_idx == 0 {
                    number_start_c_idx = cidx;
                }
            } else if number.len() > 0 {
                // These will be the search indices
                // The extra bit is because usze cannot be negative

                let mut upper_r_idx: usize = 0;
                if ridx > 0 {
                    upper_r_idx = ridx - 1;
                }
                let lower_r_idx = min(ridx + 1, rows - 1);

                let mut upper_c_idx: usize = 0;
                if number_start_c_idx > 0 {
                    upper_c_idx = number_start_c_idx - 1;
                }
                let lower_c_idx = min(cidx, cols - 1);

                let mut found = false;

                // Now I need to check the surrounding cells to see if they are occupied.
                // If they are then I need to store the number.
                for r in upper_r_idx..lower_r_idx + 1 {
                    for c in upper_c_idx..lower_c_idx + 1 {
                        // println!("Checking cell: [{}, {}]", r, c);
                        // println!("Cell value: {}", array[[r, c]]);
                        if (array[[r, c]] != '.') && (!array[[r, c]].is_digit(10)) {
                            // println!("Found a number: {}", number);

                            total += number.parse::<u32>().unwrap();

                            // Reset the number and the start index
                            number = String::new();
                            number_start_c_idx = 0;

                            found = true;
                        }
                    }
                }

                if !found {
                    println!("Did not find a number");

                    // I'm missing one so let's print the surrounding cells
                    let subarray = array.slice(s![
                        upper_r_idx..lower_r_idx + 1,
                        upper_c_idx..lower_c_idx + 1
                    ]);
                    println!("Subarray: \n{:?}", subarray);

                    // println!("Resetting number and start index");
                    // println!("Nmber is now: {}", number);
                    // printarray[[1, 3]]
                    number = String::new();
                    number_start_c_idx = 0;
                }
            }

            // if the character is *not* an ascii digit then I need to
            // check the surrounding cells to see if they are occupied.
            // If they are then I need to store the number.
        }

        // Edge case for numbers at the end of rows
        // if the number is valid
        if number.len() > 0 {
            let mut upper_r_idx: usize = 0;
            if ridx > 0 {
                upper_r_idx = ridx - 1;
            }
            let lower_r_idx = min(ridx + 1, rows - 1);

            let mut upper_c_idx: usize = 0;
            if number_start_c_idx > 0 {
                upper_c_idx = number_start_c_idx - 1;
            }
            let lower_c_idx = cols - 1;

            let mut found = false;

            // Now I need to check the surrounding cells to see if they are occupied.
            // If they are then I need to store the number.
            for r in upper_r_idx..lower_r_idx + 1 {
                for c in upper_c_idx..lower_c_idx + 1 {
                    // println!("Checking cell: [{}, {}]", r, c);
                    // println!("Cell value: {}", array[[r, c]]);
                    if (array[[r, c]] != '.') && (!array[[r, c]].is_digit(10)) {
                        // println!("Found a number: {}", number);

                        total += number.parse::<u32>().unwrap();

                        // Reset the number and the start index
                        number = String::new();
                        number_start_c_idx = 0;

                        found = true;
                    }
                }
            }

            if !found {
                println!("Did not find a number");

                // I'm missing one so let's print the surrounding cells
                let subarray = array.slice(s![
                    upper_r_idx..lower_r_idx + 1,
                    upper_c_idx..lower_c_idx + 1
                ]);
                println!("Subarray: \n{:?}", subarray);

                // println!("Resetting number and start index");
                // println!("Nmber is now: {}", number);
                // printarray[[1, 3]]
                number = String::new();
                number_start_c_idx = 0;
            }
        }
    }

    println!("Total: {}", total);
    total
}

fn challenge_2(data: &str) -> u32 {
    // Read in the data file.
    let mut values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // use a map to get the list of all the lines as chars
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // Construct an ndarray of characters from the lines
    let rows = lines.len();
    let cols = lines[0].len();

    // Initialize with empty char and fill with the
    // characters from the lines
    let mut array = Array2::from_elem((rows, cols), ' ');
    for (i, line) in lines.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            array[[i, j]] = c;
        }
    }

    // println!("{:?}", array);

    // Visit each of the cells once. Construct a number from the sequential
    // values until it hits a "." or non-numeric value. Then check the indices
    // of the surrounding cells to see if they are occupied. If they are then
    // store the number

    let mut total = 0;

    let mut first_partner = 0;

    // This is kind of absurd but I'm going to store the gear location
    // as an array where the key is the location and the value is the
    // first partner value. This is assumign no 3x numbers.
    let mut gear_locations: HashMap<[usize; 2], u32> = HashMap::new();

    for ridx in 0..rows {
        // Must be String not &str because I need it to be mutable.
        let mut number: String = String::new();

        // This will be the column index of the start of the number
        // I am using it to decide which cells to check for occupancy
        // Reset to 0 when I store or reject a number
        let mut number_start_c_idx = 0;

        for cidx in 0..cols {
            // If the char is a digit then I need to store it
            if array[[ridx, cidx]].is_ascii_digit() {
                number.push(array[[ridx, cidx]]);
                // println!("Adding digit: {}", array[[ridx, cidx]]);
                // println!("Number: {}", number);

                // If this is the first digit then I need to store the column index
                // so that I can check the surrounding cells for occupancy.
                // NOTE: I will reset this to 0 when I store or reject an umber
                if number_start_c_idx == 0 {
                    number_start_c_idx = cidx;
                }
            } else if number.len() > 0 {
                // These will be the search indices
                // The extra bit is because usze cannot be negative

                let mut upper_r_idx: usize = 0;
                if ridx > 0 {
                    upper_r_idx = ridx - 1;
                }
                let lower_r_idx = min(ridx + 1, rows - 1);

                let mut upper_c_idx: usize = 0;
                if number_start_c_idx > 0 {
                    upper_c_idx = number_start_c_idx - 1;
                }
                let lower_c_idx = min(cidx, cols - 1);

                let mut found = false;

                // Now I need to check the surrounding cells to see if they are occupied.
                // If they are then I need to store the number.
                for r in upper_r_idx..lower_r_idx + 1 {
                    for c in upper_c_idx..lower_c_idx + 1 {
                        // println!("Checking cell: [{}, {}]", r, c);
                        // println!("Cell value: {}", array[[r, c]]);
                        if array[[r, c]] == '*' {
                            // So if we find a *, we want to store the first
                            // partner number and then check the second partner

                            println!("Gears: {:?}", gear_locations);

                            // If this is the SECOND partner, then the location should
                            // already be in the hashmap. If it is not then we need to
                            // store the location and the first partner number.

                            if gear_locations.contains_key(&[r, c]) {
                                // the location is already in the hashmap so we need to
                                // multiply the two numbers and add to the total

                                println!("Found a match for gear location [{}, {}]", r, c);
                                let first_partner = gear_locations.get(&[r, c]).unwrap();
                                let second_partner = number.parse::<u32>().unwrap();

                                total += first_partner * second_partner;
                            } else {
                                // the location is not in the hashmap so we need to
                                // store the location and the first partner number.
                                println!("Storing gear location [{}, {}]", r, c);
                                gear_locations.insert([r, c], number.parse::<u32>().unwrap());
                            }

                            // Reset the number and the start index
                            number = String::new();
                            number_start_c_idx = 0;

                            found = true;
                        }
                    }
                }

                if !found {
                    // println!("Did not find a number");

                    // // I'm missing one so let's print the surrounding cells
                    // let subarray = array.slice(s![
                    //     upper_r_idx..lower_r_idx + 1,
                    //     upper_c_idx..lower_c_idx + 1
                    // ]);
                    // println!("Subarray: \n{:?}", subarray);

                    // println!("Resetting number and start index");
                    // println!("Nmber is now: {}", number);
                    // printarray[[1, 3]]
                    number = String::new();
                    number_start_c_idx = 0;
                }
            }

            // if the character is *not* an ascii digit then I need to
            // check the surrounding cells to see if they are occupied.
            // If they are then I need to store the number.
        }

        // Edge case for numbers at the end of rows
        // if the number is valid
        if number.len() > 0 {
            let mut upper_r_idx: usize = 0;
            if ridx > 0 {
                upper_r_idx = ridx - 1;
            }
            let lower_r_idx = min(ridx + 1, rows - 1);

            let mut upper_c_idx: usize = 0;
            if number_start_c_idx > 0 {
                upper_c_idx = number_start_c_idx - 1;
            }
            let lower_c_idx = cols - 1;

            let mut found = false;

            // Now I need to check the surrounding cells to see if they are occupied.
            // If they are then I need to store the number.
            for r in upper_r_idx..lower_r_idx + 1 {
                for c in upper_c_idx..lower_c_idx + 1 {
                    // println!("Checking cell: [{}, {}]", r, c);
                    // println!("Cell value: {}", array[[r, c]]);
                    if (array[[r, c]] == '*') {
                        // println!("Found a number: {}", number);

                        println!("Gears: {:?}", gear_locations);

                        if gear_locations.contains_key(&[r, c]) {
                            // the location is already in the hashmap so we need to
                            // multiply the two numbers and add to the total

                            println!("Found a match for gear location [{}, {}]", r, c);
                            let first_partner = gear_locations.get(&[r, c]).unwrap();
                            let second_partner = number.parse::<u32>().unwrap();

                            total += first_partner * second_partner;
                        } else {
                            // the location is not in the hashmap so we need to
                            // store the location and the first partner number.
                            println!("Storing gear location [{}, {}]", r, c);
                            gear_locations.insert([r, c], number.parse::<u32>().unwrap());
                        }

                        // Reset the number and the start index
                        number = String::new();
                        number_start_c_idx = 0;

                        found = true;
                    }
                }
            }

            if !found {
                // println!("Did not find a number");

                // // I'm missing one so let's print the surrounding cells
                // let subarray = array.slice(s![
                //     upper_r_idx..lower_r_idx + 1,
                //     upper_c_idx..lower_c_idx + 1
                // ]);
                // println!("Subarray: \n{:?}", subarray);

                // println!("Resetting number and start index");
                // println!("Nmber is now: {}", number);
                // printarray[[1, 3]]
                number = String::new();
                number_start_c_idx = 0;
            }
        }
    }

    println!("Total: {}", total);
    total
}
