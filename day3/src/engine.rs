use std::vec;
#[derive(Clone)]
pub struct Match {
    number: u32,
    coordinates: Vec<(usize, usize)>
}

impl Match {
    pub fn is_adjacent(&self, compared_coord: (usize, usize)) -> bool {
        for c in &self.coordinates {
            let current_coord_i = c.0;
            let current_coord_j = c.1;

            let compared_coord_i = compared_coord.0;
            let compared_coord_j = compared_coord.1;

            if current_coord_i == compared_coord_i && current_coord_j == compared_coord_j {
                continue;
            }

            if current_coord_i > 0 && current_coord_j > 0 {
                if (current_coord_i+1 == compared_coord_i || current_coord_i-1 == compared_coord_i || current_coord_i == compared_coord_i)
                    && (current_coord_j+1 == compared_coord_j || current_coord_j-1 == compared_coord_j || current_coord_j == compared_coord_j) {
                    return true;
                }
                continue;
            }

            if current_coord_j > 0 {
                if (current_coord_i+1 == compared_coord_i || current_coord_i == compared_coord_i)
                    && (current_coord_j+1 == compared_coord_j || current_coord_j-1 == compared_coord_j || current_coord_j == compared_coord_j) {
                    return true;
                }
                continue;
            }

            if current_coord_i > 0 {
                if (current_coord_i+1 == compared_coord_i || current_coord_i-1 == compared_coord_i || current_coord_i == compared_coord_i)
                    && (current_coord_j+1 == compared_coord_j || current_coord_j == compared_coord_j) {
                    return true;
                }
                continue;
            }

            if (current_coord_i+1 == compared_coord_i || current_coord_i == compared_coord_i)
                && (current_coord_j+1 == compared_coord_j || current_coord_j == compared_coord_j) {
                return true;
            }

        }
        false
    }
    pub fn get_number(self) -> u32 {
        self.number
    }
    pub fn new(number: u32, coordinates: Vec<(usize, usize)>) -> Self {
        Self { number, coordinates }
    }
}

#[derive(Clone)]
pub struct Game {
    width: usize,
    length: usize
}

impl Game {
    pub fn new() -> Self {
        Self { width:0, length:0 }
    }

    pub fn sum_parts_part1(&mut self, parts: Vec<String>) -> u32 {
        self.length = parts.len();
        self.width = parts[0].len();
        let mut m: Vec<Vec<char>> = vec![vec!['0'; self.width]; parts.len()];

        for i in 0..parts.len() {
            let chars: Vec<char> = parts[i].chars().collect();
            for (j, item) in chars.iter().enumerate() {
                match item {
                    '.' => { m[i][j] = '.'},
                    '0'..='9' => m[i][j] = *item,
                    _ => { m[i][j] = 's' }
                }
            }
        }

        let mut matches : Vec<Match> = Vec::new();
        let mut coordinates : Vec<(usize,usize)> = Vec::new();
        let mut number: String;

        for (i, line) in m.iter().enumerate() {
            number = String::new();
            let mut append_number = false;

            for j in 0..line.len() {
                let value = m[i][j];

                if is_number(value) {
                    if !append_number {
                        number = String::from(value);
                        append_number = true;
                        coordinates.push((i,j));
                        continue;
                    }

                    number = format!("{}{}",number,value);
                    coordinates.push((i,j));
                    // We need to store all coordinates where there is a number.
                    // After that, if a single symbol is adjacent to the number, number is added to part_numbers vector.
                } else {
                    if !number.is_empty(){
                        matches.push(Match::new(number.parse().unwrap(), coordinates.clone()));
                    }

                    coordinates.clear();
                    number.clear();
                    append_number = false;
                }
            }

            if !number.is_empty(){
                matches.push(Match::new(number.parse().unwrap(), coordinates.clone()));
            }
        }

        let mut part_numbers : Vec<u32> = Vec::new();
        for (i, line) in m.iter().enumerate() {
            for j in 0..line.len() {
                if is_symbol(m[i][j]) {
                    // If a single symbol is adjacent to a match, we add that number to the part_numbers array
                    // and then remove it from the Matches array.
                    matches.retain(|current_match| {
                        if current_match.clone().is_adjacent((i,j)) {
                            part_numbers.push(current_match.clone().get_number());
                            return false
                        }
                        true
                    })

                }
            }
        }

        let mut result:u32 = 0;
        for p in part_numbers {
            result += p;
        }

        result
     }
}

fn is_number(value: char) -> bool {
    value != 's' && value != '.'
}

fn is_symbol(value: char) -> bool {
    value == 's'
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_implement_adjacent_upper_left_corner(){
        let m = Match::new(1, vec![(0,0)]);

        assert_eq!(false, m.clone().is_adjacent((0,0)));
        assert_eq!(true, m.clone().is_adjacent((0,1)));
        assert_eq!(true, m.clone().is_adjacent((1,0)));
        assert_eq!(true, m.clone().is_adjacent((1,1)));
        assert_eq!(false, m.clone().is_adjacent((0,2)));
    }

    #[test]
    fn should_implement_adjacent_upper_right_corner(){
        let m = Match::new(1, vec![(0,2)]);

        assert_eq!(true, m.clone().is_adjacent((0,1)));
        assert_eq!(true, m.clone().is_adjacent((1,1)));
        assert_eq!(true, m.clone().is_adjacent((1,2)));
        assert_eq!(true, m.clone().is_adjacent((1,2)));
    }
    #[test]
    fn should_implement_adjacent_bottom_left_corner(){
        let m = Match::new(1, vec![(2,0)]);

        assert_eq!(true, m.clone().is_adjacent((1,0)));
        assert_eq!(true, m.clone().is_adjacent((1,1)));
        assert_eq!(true, m.clone().is_adjacent((2,1)));
        assert_eq!(false, m.clone().is_adjacent((2,2)));
    }

    #[test]
    fn should_implement_adjacent_bottom_right_corner(){
        let m = Match::new(1, vec![(2,2)]);

        assert_eq!(true, m.clone().is_adjacent((2,1)));
        assert_eq!(true, m.clone().is_adjacent((1,2)));
        assert_eq!(true, m.clone().is_adjacent((1,1)));
    }

    #[test]
    fn should_implement_adjacent_all(){
        let m = Match::new(1, vec![(2,2)]);

        assert_eq!(true, m.clone().is_adjacent((1,2)));
        assert_eq!(true, m.clone().is_adjacent((1,1)));
        assert_eq!(true, m.clone().is_adjacent((1,3)));
        assert_eq!(true, m.clone().is_adjacent((2,1)));
        assert_eq!(true, m.clone().is_adjacent((2,3)));
        assert_eq!(true, m.clone().is_adjacent((3,2)));
        assert_eq!(true, m.clone().is_adjacent((3,1)));
        assert_eq!(true, m.clone().is_adjacent((3,3)));
    }

    #[test]
    fn should_sum_all_parts_adjacent_to_symbols_given_example(){
        let mut game = Game::new();
        let input_str: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let lines: Vec<String> = input_str.lines().map(String::from).collect();

        assert_eq!(4361, game.sum_parts_part1(lines));
    }

    #[test]
    fn should_sum_all_parts_adjacent_to_symbols_including_numbers_in_right_corner(){
        let mut game = Game::new();

        let mut lines = Vec::new();

        lines.push(String::from("..12......"));
        lines.push(String::from("*....*1..."));
        lines.push(String::from("22.......1"));
        lines.push(String::from(".....*...*"));
        lines.push(String::from("....2.3..1"));

        // Should add all parts but 29
        // 22 + 1 + 2 + 3 + 1 + 1 = 30

        assert_eq!(30, game.sum_parts_part1(lines));
    }
}