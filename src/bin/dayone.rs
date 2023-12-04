use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::env;

#[derive(Debug)]
enum SearchDirection {
    Forwards,
    Backwards
}

const RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let read_lines = get_lines(lines);
        let part1_solution = solve_part1(read_lines.clone());
        println!(" ** part1_solution = {part1_solution} ** ");
        let part2_solution = solve_part2(read_lines);
        println!(" ** part2_solution, not correct! = {part2_solution} ** ");
    }
}

fn get_lines(lines: Lines<BufReader<File>>) -> Vec<String> {
    let mut read_lines: Vec<String> = vec![];
    for line in lines {
        if let Ok(this_line) = line {
            read_lines.push(this_line);
        }
    }

    read_lines
}

fn solve_part1(lines: Vec<String>) -> u32 {
    let mut solution: u32 = 0;

    for calibration_value in lines {
        let first_digit = find_first_digit(calibration_value.clone(), SearchDirection::Forwards).unwrap();
        let last_digit = find_first_digit(calibration_value.clone(), SearchDirection::Backwards).unwrap();
        let checksum = format!("{}{}", first_digit, last_digit);
        let checkum_number: u32 = checksum.trim().parse().unwrap();
        solution += checkum_number;
    }

    solution
}

fn solve_part2(lines: Vec<String>) -> u32 {
    let number_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut solution: u32 = 0;

    for line in lines {
        let first_digit = find_first_valid_number(&line, &number_map, SearchDirection::Forwards);
        let last_digit = find_first_valid_number(&line, &number_map, SearchDirection::Backwards);

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let checksum = format!("{}{}", first, last);
            let checksum_number: u32 = checksum.parse().unwrap();
            solution += checksum_number;
        }
    }

    solution
}

fn find_first_valid_number(line: &str, number_map: &HashMap<&str, u32>, direction: SearchDirection) -> Option<u32> {
    let mut current_word = String::new();

    let chars: Vec<char> = match direction {
        SearchDirection::Forwards => line.chars().collect(),
        SearchDirection::Backwards => line.chars().rev().collect(),
    };

    for ch in chars.iter() {
        if ch.is_alphabetic() {
            current_word.push(*ch);
            let check_word = match direction {
                SearchDirection::Forwards => current_word.clone(),
                SearchDirection::Backwards => current_word.chars().rev().collect(),
            };
            // check for partial match
            let keys: Vec<&&str> = number_map.keys().collect();
            if let Some(matching_key) = keys.iter().find(|s| check_word.starts_with(**s)) {
                // look for exact match
                if let Some(&number) = number_map.get(**matching_key) {
                    return Some(number);
                }
            }
        } else if ch.is_digit(RADIX) {
            return ch.to_digit(RADIX);
        }
    }

    None
}

fn find_first_digit(search: String, direction: SearchDirection) -> Option<u32> {
    let search_string = match direction {
        SearchDirection::Forwards => search,
        SearchDirection::Backwards => search.chars().rev().collect::<String>()
    };

    for c in search_string.chars() {
        if c.is_digit(RADIX) {
            return c.to_digit(RADIX);
        }
    }

    return None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// eof