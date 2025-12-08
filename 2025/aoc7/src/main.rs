use std::collections::HashMap;
use std::error::Error;
use std::fs;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct Point {
//     x: i64,
//     y: i64,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();
    let mut beams: Vec<(usize, i64)> = vec![];
    let mut splits: Vec<usize> = vec![];

    let first = lines.next().unwrap();

    let start = first.find("S").unwrap();
    beams.push((start, 1));

    let mut total_count = 0;

    for l in lines {
        println!("{}", l);
        splits = l.match_indices("^").map(|(i, _)| i).collect();
        // println!("splits: {:?}", splits);
        // println!("beams: {:?}", beams);
        let mut new_beams = beams.clone();
        // let mut split_count = 0;
        for (beam, count) in &beams {
            for split in &splits {
                if beam == split {
                    // println!("SPLIT at {}", beam);
                    new_beams.push((*beam + 1, *count));
                    new_beams.push((*beam - 1, *count));
                    new_beams.retain(|&(x, _)| x != *beam);
                    // split_count += 1;
                }
            }
        }
        // println!("new_beams: {:?}", new_beams);

        let mut map: HashMap<usize, i64> = HashMap::new();
        for (beam, count) in &new_beams {
            if let Some(existing_count) = map.get(beam) {
                map.insert(*beam, existing_count + count);
            } else {
                map.insert(*beam, *count);
            }
        }
        // println!("map: {:?}", map);
        beams = map.into_iter().collect();

        // beams = new_beams;
        // total_count += split_count;
    }

    let sum = beams.iter().map(|(_, count)| count).sum::<i64>();

    println!("Final beams: {:?}", beams);
    println!("Total splits: {}", sum);

    return Ok(());
}
