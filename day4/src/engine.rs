use std::ops::Deref;
use std::vec;

#[derive(Clone)]
pub struct Game {
    scratch_numbers: Vec<u32>,
    winning_numbers: Vec<u32>
}

impl Game {
    pub fn new(round_str: &str) -> Self {
        let mut scratch_numbers_u32: Vec<u32> = vec![];
        let mut winning_numbers_u32: Vec<u32> = vec![];

        let all_game_string: Vec<String> = round_str.lines().map(String::from).collect();
        all_game_string.clone().into_iter().for_each(|e| {
            let s : Vec<&str> = e.split(':').collect();
            let scratch: Vec<&str> = s[1].split('|').collect();

            scratch_numbers_u32 = scratch[0].split(' ').filter(|&x| {
                if x.eq(""){
                    return false
                }
            return true
            }).collect::<Vec<&str>>().iter().map(|&w| w.parse::<u32>().unwrap()).collect();

            winning_numbers_u32 = scratch[1].split(' ').filter(|&x| {
                if x.eq("") || x.eq(" "){
                    return false
                }
            return true
            }).collect::<Vec<&str>>().iter().map(|&w| w.parse::<u32>().unwrap()).collect();
        });

        Self { scratch_numbers: scratch_numbers_u32 , winning_numbers: winning_numbers_u32}
    }

    pub fn calculate_points(self) -> u32 {

        let matched_numbers : Vec<&u32> = self.winning_numbers.iter().filter(|&x| {
            if self.scratch_numbers.contains(&x) {
                return true
            }
            return false
        }).collect();

        if matched_numbers.len() == 0 {
            return 0;
        }

        u32::pow(2, (matched_numbers.len() - 1)  as u32)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_points_from_scratchcard() {
        let input_str: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
";
        let mut game = Game::new(input_str);

        assert_eq!(8, game.calculate_points());
    }
}