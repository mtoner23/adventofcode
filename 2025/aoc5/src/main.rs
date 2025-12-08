use regex::Regex;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    start: u64,
    end: u64,
}

fn get_range_size(r: &Range) -> u64 {
    return r.end - r.start + 1;
}

fn in_range(r: &Range, v: u64) -> bool {
    return v >= r.start && v <= r.end;
}

fn merge_ranges(r1: &Range, r2: &Range) -> Option<Range> {
    if r1.end + 1 < r2.start || r2.end + 1 < r1.start {
        return None;
    }

    let start = std::cmp::min(r1.start, r2.start);
    let end = std::cmp::max(r1.end, r2.end);

    return Some(Range { start, end });
}

fn validate_range(r: &Range) -> bool {
    return r.start <= r.end;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
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

            if !validate_range(ranges.last().unwrap()) {
                panic!("Invalid range: {}-{}", start, end);
            }
        } else {
            break;
        }
    }

    let mut new_ranges: Vec<Range> = ranges.clone();

    loop {
        let mut changes = false;
        for r1 in 0..ranges.len() {
            for r2 in r1 + 1..ranges.len() {
                let range1 = &ranges[r1];
                let range2 = &ranges[r2];
                if let Some(merged) = merge_ranges(range1, range2) {
                    println!(
                        "Merging {}-{} and {}-{} into {}-{}",
                        range1.start,
                        range1.end,
                        range2.start,
                        range2.end,
                        merged.start,
                        merged.end
                    );
                    new_ranges[r1] = merged;
                    new_ranges.remove(r2);
                    changes = true;
                    break;
                }
            }
            if changes {
                break;
            }
        }
        // println!("New ranges: {:?}", new_ranges);
        if !changes {
            break;
        } else {
            ranges = new_ranges.clone();
        }
    }

    let count = ranges.iter().map(|r| get_range_size(r)).sum::<u64>();

    println!("Count: {}", count);

    //Part 1
    // let mut count = 0;

    // for l in lines {
    //     let v: u64 = l.parse().unwrap();
    //     let mut found = false;
    //     for r in &ranges {
    //         if in_range(r, v) {
    //             found = true;
    //             break;
    //         }
    //     }
    //     if found {
    //         count += 1;
    //         // println!("{}: YES", v);
    //     } else {
    //         // println!("{}: NO", v);
    //     }
    // }

    return Ok(());
}
