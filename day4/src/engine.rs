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

    pub fn collect_winning_numbers(&self) -> Vec<&u32> {
        self.winning_numbers.iter().filter(|&x| {
            if self.scratch_numbers.contains(&x) {
                return true
            }
            return false
        }).collect()
    }

    pub fn calculate_points(self) -> u32 {
        let matched_numbers : Vec<&u32> = self.collect_winning_numbers();
        if matched_numbers.len() == 0 {
            return 0;
        }

        u32::pow(2, (matched_numbers.len() - 1)  as u32)
    }

    pub fn calculate_scratch_cards(self) -> u32 {
        let winning_numbers: Vec<&u32> = self.collect_winning_numbers();

        if winning_numbers.len() == 0 {
            return 1;
        }

        (winning_numbers.len() + 1) as u32
    }

    pub fn update_scratch_cards_iteration(self, mut iteration: usize, multipliers: &mut Vec<u32>, res: u32) {
        let multipliers_len = multipliers.clone().len()-1;

        // Repeat the count iteration n times, given that there might be several instances of each card
        for _ in 0.. multipliers[iteration] {
            for n_time in 0..res - 1 {
                let next_index = iteration + n_time as usize + 1;
                if next_index > multipliers_len {
                    continue
                }
                multipliers[next_index]+=1;
            }
        }
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
        let game = Game::new(input_str);

        assert_eq!(8, game.calculate_points());
    }

    #[test]
    fn should_calculate_scratch_cards_from_scratchcard() {
        let input_str: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let mut iteration:usize = 0;
        let mut total_cards:u32 = 0;

        let mut multipliers:Vec<u32> = vec![1; input_str.lines().count()];

        // I'm putting logic in tests, this is not nice.
        input_str.lines().for_each(|line| {
            let game = Game::new(line);

            let res = game.clone().calculate_scratch_cards();
            game.update_scratch_cards_iteration(iteration, &mut multipliers, res);
            iteration += 1;
        });

        multipliers.iter().for_each(|m| {
            total_cards += m;
        });

        assert_eq!(30, total_cards);
    }
}