use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");

    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    // let mut sum = 0;
    let mut cards: Vec<usize> = vec![];
    cards.resize(file.lines().count(), 1);
    // let mut found: Vec<usize> = vec![];
    for (l, line) in file.lines().enumerate() {
        let mut card = line.split(":").nth(1).unwrap().split("|");
        // println!("{:?}", card.clone().nth(0));
        // println!("{:?}", card.clone().nth(1));
        let answer_nums = card.nth(0).unwrap().split_whitespace();
        // println!("{:?}", answer_nums.clone().nth(0));
        // println!("{:?}", answer_nums);
        let ticket_nums = card.nth(0).unwrap().split_whitespace();

        let mut matching_nums = 0;
        for num in ticket_nums {
            // println!("Num {num}:");
            if answer_nums.clone().any(|x| x == num) {
                // println!("Found a answer: {num}, {found}, {sum}: {line}");
                matching_nums += 1;
            }
        }

        let incr = cards[l];
        for i in 0..matching_nums {
            if l + i + 1 >= cards.len() {
                break;
            }
            cards[l + i + 1] += incr;
        }

        // println!("Cards: {:?}", cards);
    }

    println!("Cards sum: {}", cards.iter().sum::<usize>());

    return Ok(());
}
