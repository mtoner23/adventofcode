use core::num;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut sum: i64 = 0;

    for l in lines {
        println!("{}", l);
        let chars: Vec<char> = l.chars().collect();

        let mut maximums: Vec<(char, usize)> = Vec::new();
        let num_values = 12;
        let prev_max_value = 0;
        let length = chars.len();

        for (pos, c) in chars[(prev_max_value)..(length - num_values)]
            .iter()
            .enumerate()
        {
            if let Some(digit) = c.to_digit(10) {
                if find_1.is_none() || digit > find_1.unwrap().0.to_digit(10).unwrap_or(0) {
                    find_1 = Some((*c, pos));
                }
            }
        }

        let (max_char, pos) = find_1.expect("No max digit found in line");

        println!("Max: {} at position {}", max_char, pos);

        for (pos_2, c) in chars[(pos + 1)..].iter().enumerate() {
            // println!("Checking char: {} at pos {}", c, pos_2);
            if let Some(digit) = c.to_digit(10) {
                if find_2.is_none() || digit > find_2.unwrap().0.to_digit(10).unwrap_or(0) {
                    find_2 = Some((*c, pos_2 + pos + 1));
                    // println!("New max found: {} at pos {}", c, pos_2 + pos + 1);
                }
            }
        }

        let (max_char_2, pos_2) = find_2.expect("No max digit found in line");

        println!("Max: {} at position {}", max_char_2, pos_2);
        let num = (max_char.to_digit(10).unwrap() * 10 + max_char_2.to_digit(10).unwrap()) as i64;

        sum += num;
    }

    println!("Sum: {}", sum);

    return Ok(());
}
