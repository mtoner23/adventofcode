use core::panic;
use geo::*;
use geo_types::coord;
use geo_types::{Coord, LineString};
use std::error::Error;
use std::fs;
use std::vec;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut start = Coord { x: 0., y: 0. };
    let mut coords: Vec<Coord<f32>> = vec![start];
    // let mut prev_direction = lines.last();

    // let mut count = 0;

    for l in lines {
        let mut splits = l.split_whitespace();
        let direction = splits.next().unwrap();
        let mut distance = splits.next().unwrap().parse::<f32>()?;
        // if count != 0 {
        //     distance += 1.0;
        // }
        let next = match direction {
            "U" => Coord {
                x: start.x,
                y: start.y - distance,
            },
            "D" => Coord {
                x: start.x,
                y: start.y + distance,
            },
            "L" => Coord {
                x: start.x - distance,
                y: start.y,
            },
            "R" => Coord {
                x: start.x + distance,
                y: start.y,
            },
            _ => {
                println!("Unknown Direction");
                panic!()
            }
        };
        // println!("{:?}", next);
        coords.push(next);
        // count += 1;
        start = next;
    }

    let line_string = LineString::new(coords.clone());
    // dbg!(&line_string);
    assert!(line_string.is_closed() == true);

    let length = line_string.euclidean_length();

    let poly = Polygon::new(line_string, vec![]);
    let area = poly.unsigned_area();

    println!("area: {}", area);
    println!("length: {}", length);
    println!("points: {}", coords.len());
    println!("Total: {}", area + length / 2. + 1.);

    return Ok(());
}
