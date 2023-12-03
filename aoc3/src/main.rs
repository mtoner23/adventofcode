// use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
struct Location {
    x: usize,
    y: usize,
}

fn main() {
    // println!("Hello, world!");

    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath).unwrap();

    // let num_regex = Regex::new(r"\d+").unwrap();

    // let height = file.lines().count();
    // let width = file.lines().nth(0).unwrap().len();

    let mut symbols: Vec<(char, Location)> = vec![];
    let mut numbers: Vec<(usize, Location, Location)> = vec![];

    let mut num: String = "".to_owned();

    // let mut total_nums: Vec<usize> = vec![];

    // for l in file.lines() {
    //     let line_nums = &num_regex.captures(l).unwrap();
    //     // let nums = line_nums["num"];
    //     for n in line_nums.iter() {
    //         println!("{}", n.unwrap().as_str());
    //     }
    // }

    for (l, line) in file.lines().enumerate() {
        for (c, character) in line.chars().enumerate() {
            if character.is_numeric() {
                if num.len() == 0 {
                    numbers.push((0, Location { x: c, y: l }, Location { x: 0, y: 0 }));
                }
                num.push(character);
            } else {
                if num.len() != 0 {
                    numbers.last_mut().unwrap().0 = num.parse().unwrap();
                    numbers.last_mut().unwrap().2 = Location { x: c - 1, y: l };
                }
                num = "".to_owned();
                if character != '.' {
                    symbols.push((character, Location { x: c, y: l }));
                }
            }
        }
        if num.len() != 0 {
            numbers.last_mut().unwrap().0 = num.parse().unwrap();
            numbers.last_mut().unwrap().2 = Location {
                x: line.len() - 1,
                y: l,
            };
        }
        num = "".to_owned();
    }

    let mut part_nums: Vec<(usize, Location)> = vec![];
    let mut sum = 0;

    for s in &symbols {
        for n in &numbers {
            let x0_diff = if s.1.x > n.1.x {
                s.1.x - n.1.x
            } else {
                n.1.x - s.1.x
            };
            let x1_diff = if s.1.x > n.2.x {
                s.1.x - n.2.x
            } else {
                n.2.x - s.1.x
            };
            let y0_diff = if s.1.y > n.1.y {
                s.1.y - n.1.y
            } else {
                n.1.y - s.1.y
            };
            let y1_diff = if s.1.y > n.2.y {
                s.1.y - n.2.y
            } else {
                n.2.y - s.1.y
            };

            // let x0_diff = s.1.x.abs_diff(n.1.x);
            // let x1_diff = s.1.y.abs_diff(n.2.x);
            // let y0_diff = s.1.y.abs_diff(n.1.y);
            // let y1_diff = s.1.y.abs_diff(n.2.y);

            if x0_diff <= 1 && y0_diff <= 1 {
                if !part_nums.contains(&(n.0, n.1.clone())) {
                    // println!("Found Begin: s {:?}, n {:?}", s, n);
                    part_nums.push((n.0, n.1.clone()));
                } else {
                    // println!("Duplicate: {:?}", n);
                }
            } else if x1_diff <= 1 && y1_diff <= 1 {
                if !part_nums.contains(&(n.0, n.1.clone())) {
                    // println!("Found End  : s {:?}, n {:?}", s, n);
                    part_nums.push((n.0, n.1.clone()));
                } else {
                    // println!("Duplicate: {:?}", n);
                }
            }
        }
    }
    // for n in &numbers {
    //     println!("{:?}", n);
    // }
    // for s in symbols {
    //     println!("{:?}", s);
    // }
    for p in &part_nums {
        sum += p.0;
    }

    println!("{sum}");
}
