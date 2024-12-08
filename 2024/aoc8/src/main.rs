use itertools::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

type Rows = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    rows: Rows,
    // visiteds: Vec<(Location, Direction)>,
    height: isize,
    width: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn make_empty_grid(height: isize, width: isize) -> Grid {
    return Grid {
        rows: vec![],
        // visiteds: vec![],
        height: height,
        width: width,
    };
}

fn add_dist(loc: &Location, dist: (isize, isize)) -> Location {
    return Location {
        x: loc.x + dist.0,
        y: loc.y + dist.1,
    };
}

fn check_bounds(loc: &Location, grid: &Grid) -> bool {
    // println!("loc : {:?}, dir: {:?}", loc, dir);
    if loc.y < 0 {
        return false;
    } else if loc.y >= grid.height {
        return false;
    } else if loc.x < 0 {
        return false;
    } else if loc.x >= grid.width {
        return false;
    } else {
        return true;
    }
}

fn print_rows(rows: &Rows) {
    for r in rows {
        for c in r {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}
fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid = make_empty_grid(0, 0);

    let mut antenna_map: HashMap<char, Vec<Location>> = HashMap::new();

    for (l, line) in lines.enumerate() {
        // println!("{}", line);
        let mut row: Vec<char> = vec![];
        for (c, test) in line.chars().enumerate() {
            row.push(test);
            if test != '.' {
                let this_loc = Location {
                    x: c as isize,
                    y: l as isize,
                };
                // match antenna_map.try_insert(test, this_loc) {
                //     Err(values) => values.value.push(this_loc),
                //     Ok(_) => (),
                // };
                match antenna_map.get_mut(&test) {
                    Some(values) => values.push(this_loc),
                    None => drop(antenna_map.insert(test, vec![this_loc])),
                }
            }
        }
        grid.rows.push(row);
    }

    grid.height = grid.rows.len() as isize;
    grid.width = grid.rows[0].len() as isize;

    println!("Height {}, Width {}", grid.height, grid.width);

    let mut result = 0;

    for (_freq, locations) in &antenna_map {
        for pair in locations.iter().combinations(2) {
            let distance = (pair[0].x - pair[1].x, pair[0].y - pair[1].y);
            let neg_dist = (-distance.0, -distance.1);
            // let mut nodes = vec![];
            let mut i = 0;
            loop {
                let test_node = add_dist(pair[0], (distance.0 * i, distance.1 * i));
                if check_bounds(&test_node, &grid) {
                    if grid.rows[test_node.y as usize][test_node.x as usize] != '#' {
                        grid.rows[test_node.y as usize][test_node.x as usize] = '#';
                        result += 1;
                    } else {
                        println!(
                            "Err Loc i{}, {:?}, {}, dist {}",
                            i,
                            test_node,
                            grid.rows[test_node.y as usize][test_node.x as usize],
                            distance.0
                        );
                    }
                } else {
                    // println!("break {:?}", test_node);
                    break;
                }
                i += 1;
            }
            i = 0;
            loop {
                let test_node = add_dist(pair[1], (neg_dist.0 * i, neg_dist.1 * i));
                if check_bounds(&test_node, &grid) {
                    if grid.rows[test_node.y as usize][test_node.x as usize] != '#' {
                        grid.rows[test_node.y as usize][test_node.x as usize] = '#';
                        result += 1;
                    }
                } else {
                    // println!("break {:?}", test_node);
                    break;
                }
                i += 1;
            }
        }
    }

    print_rows(&grid.rows);
    println!("Result {:?}", result);

    return Ok(());
}
