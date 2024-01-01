mod engine;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::engine::Game;

fn main() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./input.txt") {
        let mut input:Vec<String> = Vec::new();

        lines.flatten().for_each(|line| {
            input.push(line);
        });

        let out = game.sum_parts_part1(input);
        println!("The result is: {}", out);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
