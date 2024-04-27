mod engine;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::engine::Game;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut iteration:usize = 0;
        let mut total_cards:u32 = 0;

        let collected_lines :Vec<String> = lines.flatten().collect();
        let mut multipliers:Vec<u32> = vec![1; collected_lines.clone().len()];

        collected_lines.iter().for_each(|line| {
            let game = Game::new(line.as_str());
            let res = game.clone().calculate_scratch_cards();
            game.update_scratch_cards_iteration(iteration, &mut multipliers, res);
            iteration += 1;
        });

        multipliers.iter().for_each(|m| {
            total_cards += m;
        });
        println!("The result is: {}", total_cards);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
