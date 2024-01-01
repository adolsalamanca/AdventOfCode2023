#[derive(Clone)]
pub struct Game {
    skip_symbol: char,
    width: usize,
    length: usize
}

impl Game {
    pub fn new(skip_symbol: char, width: usize) -> Self {
        Self { skip_symbol, width, length:0 }
    }

    pub fn sum_parts(&mut self, parts: Vec<String>) -> u32 {
        self.length = parts.len();
        let mut m: Vec<Vec<char>> = vec![vec!['0'; self.width]; parts.len()];
        for i in 0..parts.len() {
            let chars: Vec<char> = parts[i].chars().collect();
            for j in 0..chars.len() {
                match chars[j] {
                    '.' => { m[i][j] = '.'},
                    '0' => { m[i][j] = '0'}
                    '1' => { m[i][j] = '1'}
                    '2' => { m[i][j] = '2'}
                    '3' => { m[i][j] = '3'}
                    '4' => { m[i][j] = '4'}
                    '5' => { m[i][j] = '5'}
                    '6' => { m[i][j] = '6'}
                    '7' => { m[i][j] = '7'}
                    '8' => { m[i][j] = '8'}
                    '9' => { m[i][j] = '9'}
                    _ => { m[i][j] = 's' }
                }
            }
        }

        let mut part_numbers : Vec<u32> = Vec::new();
        let limit_i = m.len();
        for i in 0..limit_i {
            let limit_j = m[i].len();
            for j in 0..limit_j {
                if m[i][j] == 's' {
                    part_numbers.append(&mut retrieve_numbers_around(m.clone(), i, j, limit_i, limit_j))
                    // Find all numbers around to put in the final vec
                    // ..12......
                    // *....*1...
                    // 22........
                    // ..........


                    // In this case, i=1,j=0 has a symbol.
                    // We need to analyze i=0,j=0; i=0,j=1; i=1,j=1; i=2,j=0; i=2,j=1;
                    // That is --
                    //

                    // There is a match in two of them, i=2,j=0; i=2,j=1 =>
                    // As it is contiguous, we need to concatenate chars to generate a number, 22.
                    // Now the same for i=1,j=5
                    // We need to analyze i=0,j=4; i=0,j=5; i=0,j=6; i=1,j=4; i=1,j=6; i=2,j=4; i=2,j=5; i=2,j=6;
                    // There's only one match, i=0,j=6; As it is a single match, we just add this char as uint32 to the sum of parts.
                }
            }
        }

        let mut result:u32 = 0;
        for p in part_numbers {
            result += p;
        }

        return result;
    }
}

fn retrieve_numbers_around<'a>(parts: Vec<Vec<char>>, symbol_i: usize, symbol_j: usize, limit_i: usize, limit_j: usize) -> Vec<u32> {
    println!("Coming due to symbol presence, i={},j={}",symbol_i, symbol_j);

    let mut v: Vec<u32> = Vec::new();
    let mut from_i = symbol_i;
    let mut to_i = symbol_i;
    let mut from_j = symbol_j;
    let mut to_j = symbol_j;

    if symbol_i > 0 {
        from_i = symbol_i - 1;
    }
    if symbol_i < limit_i-1 {
        to_i = symbol_i + 1;
    }

    if symbol_j > 0 {
        from_j = symbol_j - 1;
    }
    if symbol_j < limit_j-1 {
        to_j = symbol_j + 1;
    }

    let mut append_number = false;
    for i in from_i..=to_i {
        let mut number = String::new();

        for j in from_j..=to_j {
            let value = parts[i][j];
            println!("{}",value);

            if parts[i][j] == 's' || parts[i][j] == '.' {                 
                if number != "" {
                    v.push(number.parse().unwrap());
                }
                append_number = false;
            } else {
                if append_number {
                    number = format!("{}{}",number,parts[i][j]);
                    continue;
                }

                number = String::from(parts[i][j]);
                append_number = true;
            }
        }

        if number != "" {
            v.push(number.parse().unwrap());
        }
        append_number = false;
    }

    v
}

mod tests {
    use super::*;

    #[test]
    fn should_sum_all_parts_adjacent_to_symbols(){
        let mut game = Game::new('.', 10);

        let mut lines = Vec::new();
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...


        lines.push(String::from("467..114.."));
        lines.push(String::from("...*......"));
        lines.push(String::from("..35..633."));
        lines.push(String::from("......#..."));

        // Should add all parts but 114
        // 467 + 35 + 633 = 1135

        assert_eq!(1135, game.sum_parts(lines));
    }

    #[test]
    fn should_sum_all_parts_adjacent_to_symbols_start_new(){
        let mut game = Game::new('.', 10);

        let mut lines = Vec::new();
        // ..12......
        // *....*1...
        // 22........
        // ..........


        lines.push(String::from("..12......"));
        lines.push(String::from("*....*1..."));
        lines.push(String::from("22........"));
        lines.push(String::from(".....*...*"));
        lines.push(String::from("....2.3..1"));

        // Should add all parts but 29
        // 22 + 1 + 2 + 3 + 1 = 29

        assert_eq!(29, game.sum_parts(lines));
    }
}