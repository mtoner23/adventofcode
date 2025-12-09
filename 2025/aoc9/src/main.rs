use std::error::Error;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Locaiton {
    x: i64,
    y: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut locations: Vec<Locaiton> = Vec::new();

    for l in lines {
        println!("{}", l);
        let splits = l.split(",").collect::<Vec<&str>>();
        let loc = Locaiton {
            x: splits[0].parse::<i64>()?,
            y: splits[1].parse::<i64>()?,
        };
        locations.push(loc);
    }

    println!("{:?}", locations);

    let mut max = 0;

    for l1 in 0..locations.len() {
        for l2 in l1 + 1..locations.len() {
            let loc1 = &locations[l1];
            let loc2 = &locations[l2];

            let dx = (loc2.x - loc1.x).abs() + 1;
            let dy = (loc2.y - loc1.y).abs() + 1;
            let area = dx * dy;
            // println!(
            //     "Area {} between loc1: {:?} and loc2: {:?}",
            //     area, loc1, loc2
            // );
            if area > max {
                max = area;
            }
        }
        // println!("{:?}", loc);
    }

    println!("Max area: {}", max);

    return Ok(());
}
