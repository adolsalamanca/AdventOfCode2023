use std::collections::HashMap;
use regex::{Regex};

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
    fn is_valid_game(self, input: &str) -> (bool, u32) {
        let occurrences : Vec<&str> = self.clone().extract_colors(input);

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

        (true, self.game_identifier)
    }

    pub fn minimum_balls_power(self, input: &str) -> u32 {
        let occurrences : Vec<&str> = self.clone().extract_colors(input);

        let mut minimum_blue:u32 = 0;
        let mut minimum_red:u32 = 0;
        let mut minimum_green:u32 = 0;

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
                        if count > minimum_blue {
                            minimum_blue = count;
                        }
                    }
                    "red" => {
                        if count > minimum_red {
                            minimum_red = count;
                        }
                    }
                    "green" => {
                        if count > minimum_green {
                            minimum_green = count;
                        }
                    }

                    &_ => {}
                }
            }
        }

        minimum_blue*minimum_green*minimum_red
    }


    pub fn extract_colors(mut self, input: &str) -> Vec<&str> {
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
        matches[0].1.split(';').collect()
    }



    pub fn play(self, input: &str) -> u32 {
        let x = self.is_valid_game(input);
        x.1
    }
}

mod tests {
    use super::*;

    #[test]
    fn should_return_return_the_power_of_minimum_colors_required(){
        let mut game = Game::new(10, 10, 10);

        assert_eq!(120, game.minimum_balls_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 5 green"));
    }

    #[test]
    fn should_return_true_given_a_valid_game_as_string(){
        let mut game = Game::new(10, 10, 10);

        assert_eq!(true, game.is_valid_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 5 green").0);
    }
    #[test]
    fn should_return_false_given_an_invalid_game_as_string(){

        let mut game = Game::new(10, 10, 10);

        assert_eq!(false, game.is_valid_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 5 green, 6 green").0);
    }

}