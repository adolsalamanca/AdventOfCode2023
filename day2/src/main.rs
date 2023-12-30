mod engine;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::engine::Game;

fn main() {
    let game = Game::new(14,12, 13);
    if let Ok(lines) = read_lines("./input.txt") {
        let mut out:u32=0;
        for result in lines {
            if let Ok(line) = result {
                // out += game.clone().play(line.as_str());
                out += game.clone().minimum(line.as_str());
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
