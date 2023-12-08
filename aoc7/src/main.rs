use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    for line in lines {
        let mut split = line.split_whitespace();
        let hand = Hand {
            cards: split.next().unwrap().to_string(),
        };
        let bid: usize = split.next().unwrap().parse::<usize>().unwrap();
        println!("h: {:?}, b {:?}", hand, bid);
    }

    return Ok(());
}
