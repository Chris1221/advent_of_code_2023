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

fn challenge_1(data: &str) -> u32 {
    // Read in the data file.
    let mut values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the file line by line into a vector of strings.
    for line in reader.lines() {
        values.push(line.unwrap());
    }

    let mut total = 0;

    // Now for each line I need to iterate through the chars and see if
    // it is a digit or a letter. If it is a digit then I need to store it
    // to a string representation of the number.
    for line in values {
        // This is a String not a &str because I need it
        // to be mutable.
        let mut number: String = String::new();

        for char in line.chars() {
            // If the char is a digit then I need to store it
            if char.is_digit(10) {
                number.push(char);
            }
        }

        // Take the first and last chars from the string
        // to form the number.
        let first_char = number.chars().next().unwrap();
        let last_char = number.chars().last().unwrap();

        let mut real_number: String = String::new();
        real_number.push(first_char);
        real_number.push(last_char);

        let value: u32 = real_number.parse::<u32>().unwrap();

        print!(
            "For line {line} the value is {value}\n",
            line = line,
            value = value
        );

        total += value;
    }

    print!("Day 1 challenge 1: {total}", total = total);

    return total;
}

fn parse_number(number: &str) -> char {
    let mut value: &str = "0";

    match number {
        "one" => value = "1",
        "two" => value = "2",
        "three" => value = "3",
        "four" => value = "4",
        "five" => value = "5",
        "six" => value = "6",
        "seven" => value = "7",
        "eight" => value = "8",
        "nine" => value = "9",
        "zero" => value = "0",
        _ => value = "0",
    }

    return value.chars().next().unwrap();
}

fn challenge_2(data: &str) -> u32 {
    // Read in the data file.
    let mut values: Vec<String> = Vec::new();
    let file = File::open(data).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the file line by line into a vector of strings.
    for line in reader.lines() {
        values.push(line.unwrap());
    }

    let mut total = 0;

    // Now for each line I need to iterate through the chars and see if
    // it is a digit or a letter. If it is a digit then I need to store it
    // to a string representation of the number.
    for line in values {
        // This is a String not a &str because I need it
        // to be mutable.
        let mut number: String = String::new();

        // Iterate through the string and find either i)
        // a digit, or ii) a string representation of a
        // digit. If either is found as a substring then
        // flush the buffer

        let mut buffer: String = String::new();

        let numbers: Vec<&str> = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
        ];

        for char in line.chars() {
            // This is the return early case
            // where the char is a digit.
            if char.is_digit(10) {
                number.push(char);
                buffer = "".to_string();
            }

            // Now we need to add the char to the buffer
            // and check if the buffer is a number.
            buffer.push(char);

            // Does the buffer contain any of the substrings
            // in the numbers vector?
            for number_str in &numbers {
                if buffer.contains(number_str) {
                    // If it does then we need to flush the buffer
                    // and add the number to the number string.
                    let numeric_value = parse_number(number_str);

                    // Now we need to account for overlapping entries.
                    // Lets flush the buffer up to the start of the
                    // number string and then 1 more to destroy that
                    // match.
                    let index = buffer.find(number_str).unwrap() + 1;
                    buffer = buffer[index..].to_string();

                    println!("Flushing buffer, it is now: {}", buffer);

                    // First convert
                    number.push(numeric_value);
                    // buffer = "".to_string();

                    println!("Found a number: {} in line {}", numeric_value, line);
                    println!("Number is {}", number);
                }
            }
        }

        println!("Number is {}", number);

        // Take the first and last chars from the string
        // to form the number.
        let first_char = number.chars().next().unwrap();
        let last_char = number.chars().last().unwrap();

        let mut real_number: String = String::new();
        real_number.push(first_char);
        real_number.push(last_char);

        let value: u32 = real_number.parse::<u32>().unwrap();

        print!(
            "For line {line} the value is {value}\n",
            line = line,
            value = value
        );

        total += value;
    }

    print!("Day 1 challenge 1: {total}", total = total);

    return total;
}
