use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    // let my_regex = Regex::new(r"\(\#(?<dist>[0-9a-f]{5})(?<dir>[0-9])\)").unwrap();
    let my_regex =
        Regex::new(r"(?<multiply>mul\(([0-9]+),([0-9]+)\))|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();
    let mut result = 0;

    let mut enable = true;

    for l in lines {
        println!("{}", l);
        let captures = my_regex.captures_iter(l);
        // for cap in captures {
        for cap in captures {
            println!("{:?}", cap);
            if cap.name("dont").is_some() {
                enable = false;
            }
            if cap.name("do").is_some() {
                enable = true
            }

            if enable && cap.name("multiply").is_some() {
                let a = cap[2].parse::<usize>().unwrap();
                let b = cap[3].parse::<usize>().unwrap();
                println!("trying to mult: {:?}, a {}, b {}", cap[0].to_string(), a, b);
                result += a * b;
            }
        }
    }

    println!("result: {result}");

    return Ok(());
}
