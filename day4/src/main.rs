mod engine;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::engine::Game;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut points :u32 = 0;
        lines.flatten().for_each(|line| {
            let game = Game::new(line.as_str());
            let p : u32 = game.calculate_points();
            // TODO, fix failure in card 22 or 23, there's a panic.
            points += p;
        });

        println!("The result is: {}", points);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
