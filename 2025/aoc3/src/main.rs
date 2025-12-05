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

        // let mut maximums: Vec<char> = Vec::new();
        let num_values = 12;
        let mut start_pos = 0;
        let length = chars.len();
        let mut number: i64 = 0;

        for i in 0..num_values {
            let mut find: Option<(char, usize)> = None;

            println!(
                "Loop! Start pos: {}, end pos {}",
                start_pos,
                length - num_values + i + 1
            );

            for (pos, c) in chars[(start_pos)..(length - num_values + i + 1)]
                .iter()
                .enumerate()
            {
                // println!("Checking char: {} at pos {},", c, pos + start_pos);
                if let Some(digit) = c.to_digit(10) {
                    if find.is_none() || digit > find.unwrap().0.to_digit(10).unwrap_or(0) {
                        find = Some((*c, pos + start_pos));
                    }
                }
            }
            let (max, pos) = find.expect("No max digit found in line");
            println!("Max: {} at position {}", max, pos);
            start_pos = pos + 1;

            // maximums.push(max);
            number = (number * 10) + max.to_digit(10).unwrap() as i64;
        }

        // println!("Maximums: {:?}", maximums);
        println!("Number: {}", number);
        sum += number;
    }

    println!("Sum: {}", sum);

    return Ok(());
}
