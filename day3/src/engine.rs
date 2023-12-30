use std::collections::HashMap;

#[derive(Clone)]
pub struct Game {
    skip_symbol: String
}

impl Game {
    pub fn new(skip_symbol: String) -> Self {
        Self { skip_symbol }
    }

    pub fn sum_parts(&self, p0: Vec<String>) -> u32 {
        0
    }

}

mod tests {
    use super::*;

    #[test]
    fn should_sum_all_parts_adjacent_to_symbols(){
        let mut game = Game::new(String::from('.'));

        let mut lines = Vec::new();
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..

        lines.push(String::from("467..114.."));
        lines.push(String::from("...*......"));
        lines.push(String::from("..35..633."));


        // Should add all parts but 114
        // 467 + 35 + 633 = 1135

        assert_eq!(1135, game.sum_parts(lines));
    }
}