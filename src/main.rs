use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./test.txt") {
        // Consumes the iterator, returns an (Optional) String
        for result in lines {
            if let Ok(line) = result {
                println!("{}", line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn find_first_and_last_digit(input: &str) -> (u32, u32) {
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
                } else { 
                    last_digit = i;
                }
            }
        }
    }

    (first_digit, last_digit)
}

#[test]
fn test_find_first_and_last_digit() {
    let str = "12345";
    let digits= find_first_and_last_digit(str);

    assert_eq!(digits, (1,5));
}

#[test]
fn test_find_first_and_last_digit_with_letters() {
    let str = "1x12312338zq";
    let digits= find_first_and_last_digit(str);

    assert_eq!(digits, (1,8));
}