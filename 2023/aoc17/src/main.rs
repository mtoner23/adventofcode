use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    for l in lines {
        println!("{}", l);
    }

    return Ok(());
}
