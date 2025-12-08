use regex::Regex;
use std::error::Error;
use std::fs;

pub struct Range {
    start: u64,
    end: u64,
}

fn in_range(r: &Range, v: u64) -> bool {
    return v >= r.start && v <= r.end;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();

    let mut ranges: Vec<Range> = Vec::new();

    loop {
        let l = match lines.next() {
            Some(line) => line,
            None => break,
        };
        println!("{}", l);
        if l.contains("-") {
            let caps = re.captures(l).unwrap();
            let start: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let end: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            ranges.push(Range { start, end });

            // println!("Range: {}-{}", range.start, range.end);
        } else {
            break;
        }
    }

    // lines.next();

    let mut count = 0;

    for l in lines {
        let v: u64 = l.parse().unwrap();
        let mut found = false;
        for r in &ranges {
            if in_range(r, v) {
                found = true;
                break;
            }
        }
        if found {
            count += 1;
            // println!("{}: YES", v);
        } else {
            // println!("{}: NO", v);
        }
    }

    println!("Count: {}", count);

    return Ok(());
}
