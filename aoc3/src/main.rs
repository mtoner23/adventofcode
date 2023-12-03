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

    // let height = file.lines().count();
    // let width = file.lines().nth(0).unwrap().len();

    let mut symbols: Vec<(char, Location)> = vec![];
    let mut numbers: Vec<(usize, Location, Location)> = vec![];

    let mut num: String = "".to_owned();

    // let mut total_nums: Vec<usize> = vec![];

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

    // let mut part_nums: Vec<(usize, Location)> = vec![];
    let mut gears: Vec<(usize, Location)> = vec![];
    let mut sum = 0;

    for s in &symbols {
        let mut s_found = 0;
        for n in &numbers {
            let y0_diff = s.1.y.abs_diff(n.1.y);

            if y0_diff > 1 {
                continue;
            }

            let x0_diff = s.1.x.abs_diff(n.1.x);
            let x1_diff = s.1.x.abs_diff(n.2.x);
            // let y1_diff = s.1.y.abs_diff(n.2.y);

            if (x0_diff <= 1 || x1_diff <= 1) && y0_diff <= 1 {
                if !gears.contains(&(n.0, n.1.clone())) {
                    // println!("Found Begin: s {:?}, n {:?}", s, n);
                    gears.push((n.0, n.1.clone()));
                    if s.0 == '*' {
                        s_found += 1;
                    }
                } else {
                    // println!("Duplicate: {:?}", n);
                }
            }
        }
        if s_found == 2 {
            let gear_ratio = gears.pop().unwrap().0 * gears.pop().unwrap().0;
            sum += gear_ratio;
        }
    }
    // for n in &numbers {
    //     println!("{:?}", n);
    // }
    // for s in symbols {
    //     println!("{:?}", s);
    // }
    // for p in &part_nums {
    //     sum += p.0;
    // }

    println!("{sum}");
}
