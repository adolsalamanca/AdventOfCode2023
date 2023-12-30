use std::collections::HashMap;
use regex::{Regex, Split};

#[derive(Clone)]
pub struct Game {
    game_identifier: u32,
    blue_limit: u32,
    red_limit: u32,
    green_limit: u32,
}

impl Game {
    pub fn new(blue_limit: u32, red_limit: u32, green_limit: u32) -> Self {
        Self { game_identifier:0, blue_limit, red_limit, green_limit }
    }
    fn eval(mut self, input: &str) -> (bool, u32) {
        let re = Regex::new(r"Game\s(?P<game>\d+):\s+(?P<counts_colors>(?:\s*(?P<count>\d+)\s+(?P<color>\w+),*\s*;*)+)").unwrap();

        let matches: Vec<_> = re
            .captures_iter(input)
            .flat_map(|caps| {
                caps.name("game")
                    .and_then(|game| caps.name("counts_colors")
                        .map(|counts_colors| (game.as_str(), counts_colors.as_str())))
            })
            .collect();

        self.game_identifier = matches[0].0.parse().unwrap();

        let x: &str = matches[0].1;
        let occurrences : Vec<&str> = x.split(";").collect();


        for occurrence in occurrences {
            let counts_colors_re = Regex::new(r"(?P<count>\d+)\s+(?P<color>\w+),*\s*").unwrap();
            let mut colors_map: HashMap<&str, u32> = HashMap::new();

            for caps in counts_colors_re.captures_iter(occurrence) {
                if let (Some(count), Some(color)) = (caps.name("count"), caps.name("color")) {
                    let count: u32 = count.as_str().parse().unwrap_or(0);
                    *colors_map.entry(color.as_str()).or_insert(0) += count;
                }
            }

            for (color, count) in colors_map {
                match color {
                    "blue" => {
                        if count > self.blue_limit {
                            return (false, 0)
                        }
                    }
                    "red" => {
                        if count > self.red_limit {
                            return (false, 0)
                        }
                    }
                    "green" => {
                        if count > self.green_limit {
                            return (false, 0)
                        }
                    }

                    &_ => {}
                }
            }
        }

        return (true, self.game_identifier)
    }

    pub fn play(mut self, input: &str) -> u32 {
        let x = self.eval(input);
        x.1
    }
}

mod tests {
    use super::*;

    #[test]
    fn should_return_true_given_a_valid_game(){
        let game = Game::new(10,10,10);

        assert_eq!(true, game.evaluate(5,4,3));
    }

    #[test]
    fn should_return_false_given_an_invalid_game(){
        let game = Game::new(10,10,10);

        assert_eq!(false, game.evaluate(15,9,8));
    }

    #[test]
    fn should_return_true_given_a_valid_game_as_string(){
        let mut game = Game::new(10, 10, 10);

        assert_eq!(true, game.eval("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 5 green"));
    }
    #[test]
    fn should_return_false_given_an_invalid_game_as_string(){

        let mut game = Game::new(10, 10, 10);

        assert_eq!(false, game.eval("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 5 green, 6 green"));
    }

}