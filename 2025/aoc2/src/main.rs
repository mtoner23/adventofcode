use std::error::Error;
use std::fs;
use std::time::Instant;

fn is_repeating(s: &str, n: usize) -> bool {
    if s.len() % n != 0 {
        return false;
    }
    let first = &s[..n];
    s.as_bytes()
        .chunks(n)
        .all(|chunk| chunk == first.as_bytes())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut rust_sum: i64 = 0;
    let mut regex_sum: i64 = 0;

    let mut rust_total: std::time::Duration = std::time::Duration::new(0, 0);
    let mut regex_total: std::time::Duration = std::time::Duration::new(0, 0);

    let pattern = r"^(.+)\1+$";
    let regex = fancy_regex::Regex::new(&pattern).unwrap();

    for l in lines {
        println!("{}", l);
        let ranges = l.split(',');
        for r in ranges {
            let bounds = r.split('-').collect::<Vec<&str>>();
            println!("{:?}", bounds);
            let start = bounds[0].parse::<i64>().unwrap();
            let end = bounds[1].parse::<i64>().unwrap();
            for val in start..=end {
                let val_s = val.to_string();
                let len = val_s.len();
                let rust_start: Instant = Instant::now();
                for sub_len in 1..=len / 2 {
                    // Rust method
                    if is_repeating(&val_s, sub_len) {
                        // println!("MATCH: {}", val_s);
                        rust_sum += val;
                        break;
                    }
                }
                let rust_duration: std::time::Duration = rust_start.elapsed();

                let regex_start = Instant::now();
                if regex.is_match(&val_s)? {
                    // println!("MATCH: {}", val_s);
                    regex_sum += val;
                }
                let regex_duration = regex_start.elapsed();

                rust_total += rust_duration;
                regex_total += regex_duration;
            }
        }
        println!("Rust Sum: {}", rust_sum);
        println!("Regex Sum: {}", regex_sum);

        println!("Rust Time: {:?}", rust_total);
        println!("Regex Time: {:?}", regex_total);
    }

    return Ok(());
}
