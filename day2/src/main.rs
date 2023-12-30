mod engine;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut out:u32=0;
        for result in lines {
            if let Ok(line) = result {
                // out += find_first_and_last_digit_second_part(line.as_str());
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

/*
#[test]
fn test_find_first_and_last_digit_as_letters_corner_case() {
    let str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let digits= evaluate_game(str);

    assert_eq!(digits, 98);
}
*/