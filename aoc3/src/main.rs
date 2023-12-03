use std::fs;

#[derive(Debug)]
struct Location {
    x: usize,
    y: usize,
}

fn main() {
    // println!("Hello, world!");

    let filepath = "src/test.txt";
    println!("in file {} ", filepath);

    let file = fs::read_to_string(filepath).unwrap();

    // let height = file.lines().count();
    // let width = file.lines().nth(0).unwrap().len();

    let mut symbols: Vec<Location> = vec![];
    let mut numbers: Vec<(Location, Location)> = vec![];

    let mut num: String = "".to_owned();

    for (l, line) in file.lines().enumerate() {
        for (c, character) in line.chars().enumerate() {
            if character.is_numeric() {
                if num.len() == 0 {
                    numbers.push((Location { x: c, y: l }, Location { x: 0, y: 0 }));
                }
                num.push(character);
            } else {
                if num.len() != 0 {
                    numbers.last_mut().unwrap().1 = Location { x: c, y: l };
                }
                num = "".to_owned();
                if character != '.' {
                    symbols.push(Location { x: c, y: l });
                }
            }
        }
    }

    for s in &symbols {
        for n in &numbers {
            let x0_diff = if s.x > n.0.x {
                s.x - n.0.x
            } else {
                n.0.x - s.x
            };
            let x1_diff = if s.x > n.1.x {
                s.x - n.1.x
            } else {
                n.1.x - s.x
            };
            let y0_diff = if s.x > n.0.y {
                s.x - n.0.y
            } else {
                n.0.y - s.x
            };
            let y1_diff = if s.x > n.1.y {
                s.x - n.1.y
            } else {
                n.1.y - s.x
            };

            if x0_diff <= 1 && y0_diff <= 1 {
                println!("Found Begin: s {:?}, n {:?}", s, n.0);
            } else if x1_diff <= 1 && y1_diff <= 1 {
                println!("Found End  : s {:?}, n {:?}", s, n.1);
            }
        }
    }
    for n in numbers {
        println!("{:?}", n);
    }
    for s in symbols {
        println!("{:?}", s);
    }
}
