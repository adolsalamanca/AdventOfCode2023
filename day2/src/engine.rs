use regex::Regex;

pub struct Game {
    identifier: u32,
    blue_limit: u32,
    red_limit: u32,
    green_limit: u32,
}

impl Game {
    pub fn new(blue_limit: u32, red_limit: u32, green_limit: u32) -> Self {
        Self { identifier:0, blue_limit, red_limit, green_limit }
    }

    pub fn evaluate_str(&mut self, input: &str) -> bool {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let re = Regex::new(r"Game\s(?P<game>\d+):\s+(?:(?P<count>\d+)\s+(?P<color>\w+),*\s*)+");
        re.unwrap().captures_iter(input).map(|c| c.)

        self.identifier = 1;
        return true
    }

    pub fn evaluate(self, blue_balls: u32, red_balls: u32, green_balls: u32) -> bool {
        self.blue_limit >= blue_balls && self.red_limit >= red_balls && self.green_limit >= green_balls
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

        assert_eq!(true, game.evaluate_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 5 green"));
    }
    #[test]
    fn should_return_false_given_an_invalid_game_as_string(){

        let mut game = Game::new(10, 10, 10);

        assert_eq!(false, game.evaluate_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 12 green"));
    }

}


