use core::panic;
use geo::*;
use geo_types::{Coord, LineString};
use regex::Regex;
use std::error::Error;
use std::fs;
use std::vec;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut start = Coord { x: 0., y: 0. };
    let mut coords: Vec<Coord<f64>> = vec![start];

    let my_regex = Regex::new(r"\(\#(?<dist>[0-9a-f]{5})(?<dir>[0-9])\)").unwrap();

    for l in lines {
        let Some(captures) = my_regex.captures(l) else {
            println!("Err can not regex: {l}");
            return Err(Box::new(core::fmt::Error));
        };

        let dist = &captures["dist"];
        let dir = &captures["dir"];

        let direction = match dir {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => {
                panic!();
            }
        };
        let distance = i64::from_str_radix(dist, 16).unwrap() as f64;

        // println!("{direction} {distance}");

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
    // println!("points: {}", coords.len());
    println!("Total: {}", area + length / 2. + 1.);

    return Ok(());
}
