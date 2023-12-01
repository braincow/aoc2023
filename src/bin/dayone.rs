use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

enum SearchDirection {
    Forwards,
    Backwards
}

const RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut solution: u32 = 0;

    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(calibration_value) = line {
                let first_digit = find_first_digit(calibration_value.clone(), SearchDirection::Forwards).unwrap();
                let last_digit = find_first_digit(calibration_value.clone(), SearchDirection::Backwards).unwrap();
                let checksum = format!("{first_digit}{last_digit}");
                println!(" {calibration_value}: {first_digit}{last_digit} == {checksum}");
                let checkum_number: u32 = checksum.trim().parse().unwrap();
                solution += checkum_number;
            } else {
                dbg!("{line}");
            }
        }
    }

    println!("final checksum = {solution}");
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