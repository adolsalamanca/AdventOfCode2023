use regex::{Regex};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut out:u32=0;
        for result in lines {
            if let Ok(line) = result {
                out += find_first_and_last_digit_second_part(line.as_str());
            }
        }

        println!("The result is: {}",out);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first_and_last_digit_first_part(input: &str) -> u32 {
    let mut number_found = false;
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    for c in input.chars(){
        match c.to_digit(10) {
            None => (),
            Some(i) => {
                if !number_found {
                    first_digit = i;
                    number_found = true;
                    ()
                }
                
                last_digit = i;
            }
        }
    }

    first_digit*10 + last_digit
}


fn find_first_and_last_digit_second_part(input: &str) -> u32 {
    let mut number_found = false;
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    // Overlapping numbers: twone, oneight, threeight, sevenine, fiveight, eighthree, eightwo, nineight
    let re = Regex::new(r"twone|oneight|threeight|fiveight|sevenine|eighthree|eightwo|nineight|one|two|three|four|five|six|seven|eight|nine|ten|\d").unwrap();
    let matches : Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    for i in matches {
        if !number_found {
            first_digit = parse_first_number(i);
            number_found = true;
        }

        last_digit = parse_last_number(i);
    }

    first_digit*10 + last_digit
}

fn parse_first_number(p0: &str) -> u32 {
    return match p0 {
        "one" | "oneight"| "1" => { 1 }
        "two" | "twone" | "2" => { 2 }
        "three" | "threeight" | "3" => { 3 }
        "four" | "4" => { 4 }
        "five" | "fiveight" | "5" => { 5 }
        "six" | "6" => { 6 }
        "seven" | "sevenine" | "7" => { 7 }
        "eight" | "eighthree" | "eightwo" | "8" => { 8 }
        "nine" | "nineight" | "9" => { 9 }
        _ => { 0 }
    }
}
fn parse_last_number(p0: &str) -> u32 {
    return match p0 {
        "one" | "twone" | "1" => { 1 }
        "two" | "eightwo" | "2" => { 2 }
        "three" | "eighthree" |"3" => { 3 }
        "four" | "4" => { 4 }
        "five" | "5" => { 5 }
        "six" | "6" => { 6 }
        "seven" | "7" => { 7 }
        "eight" | "oneight" | "threeight" | "fiveight" | "nineight"| "8" => { 8 }
        "nine" | "sevenine" | "9" => { 9 }
        _ => { 0 }
    }
}


#[test]
fn test_find_first_and_last_digit_as_letters_corner_case() {
    let str = "znrprxdtp9sevenoneightnk";
    let digits= find_first_and_last_digit_second_part(str);

    assert_eq!(digits, 98);
}

#[test]
fn test_find_first_and_last_digit_as_letters() {
    let str = "six3x12312338zq";
    let digits= find_first_and_last_digit_second_part(str);

    assert_eq!(digits, 68);
}

#[test]
fn test_find_first_and_last_digit_as_letters_two() {
    let str = "7pqrstsixteen";
    let digits= find_first_and_last_digit_second_part(str);

    assert_eq!(digits, 76);
}


#[test]
fn test_find_first_and_last_digit_with_letters() {
    let str = "3x12312338zq";
    let digits= find_first_and_last_digit_second_part(str);

    assert_eq!(digits, 38);
}

#[test]
fn test_find_first_and_last_digit_with_letters_first_part() {
    let str = "3x12312338zq";
    let digits= find_first_and_last_digit_first_part(str);

    assert_eq!(digits, 38);
}


#[test]
fn test_find_first_and_last_digit_first_part() {
    let str = "12345";
    let digits= find_first_and_last_digit_first_part(str);

    assert_eq!(digits, 15);
}
