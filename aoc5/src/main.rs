use std::error::Error;
use std::fmt::Error as FmtError;
use std::fs;

#[derive(Debug)]
struct Range {
    input: usize,
    output: usize,
    length: usize,
}

type Mapping = Vec<Range>;

fn map_range(start: usize, end: usize, input: usize, output: usize) -> (usize, usize) {
    assert!(start >= input, "Err: S {start}, I {input}");
    return (start - input + output, end - input + output);
    // return (start, end);
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace();
    // println!("{:?}", seeds);

    let mut mappings: Vec<Mapping> = vec![];
    let mut map: Mapping = vec![];

    lines.next();

    for (_l, line) in lines.enumerate() {
        let first_char = line.chars().nth(0);
        if first_char.is_some() {
            let char: char = first_char.unwrap();
            if char.is_numeric() {
                // println!("first char num: {}", char);
                let mut nums = line.split_whitespace();
                if nums.clone().count() > 3 {
                    return Err(Box::new(FmtError));
                }
                map.push(Range {
                    output: nums.next().unwrap().parse().unwrap(),
                    input: nums.next().unwrap().parse().unwrap(),
                    length: nums.next().unwrap().parse().unwrap(),
                });
                // println!("R: {:?}", map);
            } else {
                // println!("first char letter: {}", char);
            }
        } else {
            mappings.push(map);
            map = vec![];
        }
    }
    mappings.push(map);

    for m in mappings.iter() {
        println!("Mappings: {:?}", m);
    }

    // let mut all_seeds: HashSet<usize> = HashSet::new();
    let mut all_seeds: Vec<(usize, usize)> = vec![];
    while let Some(s) = seeds.next() {
        let start_seed = s.parse::<usize>()?;
        let length = seeds.next().unwrap().parse::<usize>().unwrap();
        // let mut v: Vec<usize> = (start_seed..(start_seed + length)).collect();
        // println!("{:?}", v);
        all_seeds.push((start_seed, start_seed + length - 1));
    }

    println!("Begin Seeds: {:?}", all_seeds);
    for map in mappings.iter() {
        let mut new_seeds: Vec<(usize, usize)> = vec![];
        // println!("New Mapping");

        while let Some(seed) = all_seeds.pop() {
            let seed_start = seed.0;
            let seed_end = seed.1;
            // println!("Seed,  S: {}, E: {}", seed_start, seed_end);
            let mut found = false;
            for range in map.iter() {
                // let mut value = seed_start;
                let range_start = range.input;
                let range_end = range_start + range.length - 1;
                // println!("Range I, S: {}, E: {}", range_start, range_end,);
                if (range_start <= seed_start) && (seed_end <= range_end) {
                    // println!("Seed range entirely within this range");
                    found = true;
                    new_seeds.push(map_range(seed_start, seed_end, range.input, range.output));
                    break;
                } else if (seed_start <= range_start)
                    && (seed_end >= range_start)
                    && (seed_end <= range_end)
                {
                    // println!("Seed to the left of range");
                    found = true;
                    all_seeds.push((seed_start, range_start - 1));
                    new_seeds.push(map_range(range_start, seed_end, range.input, range.output));
                    break;
                } else if (range_start <= seed_start)
                    && (seed_start <= range_end)
                    && (range_end <= seed_end)
                {
                    // println!("Seed to the right of range");
                    found = true;
                    new_seeds.push(map_range(seed_start, range_end, range.input, range.output));
                    all_seeds.push((range_end + 1, seed_end));
                    break;
                } else if seed_start <= range_start && range_end <= seed_end {
                    // println!("Seed surrounds the range, 3 pushes");
                    found = true;
                    all_seeds.push((seed_start, range_start - 1));
                    new_seeds.push(map_range(range_start, range_end, range.input, range.output));
                    all_seeds.push((range_end + 1, seed_end));
                    break;
                }
            }
            if !found {
                new_seeds.push((seed_start, seed_end));
            }
            // println!("New:   {:?}", new_seeds);
            // break;
        }
        all_seeds = new_seeds;
    }

    let mut min = all_seeds[0].0;
    for o in &all_seeds {
        if o.0 < min {
            min = o.0;
        }
    }
    println!("End: {}", min);

    // println!("{}", outputs.iter().min().unwrap());

    return Ok(());
}
