use itertools::Itertools;
use std::error::Error;
use std::fs;

type Rows = Vec<Vec<u32>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    rows: Rows,
    // visiteds: Vec<(Location, Direction)>,
    height: isize,
    width: isize,
    end_locations: Vec<Location>,
}

impl Grid {
    fn check_bounds(&self, loc: &Location) -> bool {
        // println!("loc : {:?}, dir: {:?}", loc, dir);
        if loc.y < 0 {
            return false;
        } else if loc.y >= self.height {
            return false;
        } else if loc.x < 0 {
            return false;
        } else if loc.x >= self.width {
            return false;
        } else {
            return true;
        }
    }

    fn get_value(&self, loc: Location) -> Option<u32> {
        let out = match self.check_bounds(&loc) {
            true => Some(self.rows[loc.y as usize][loc.x as usize]),
            false => None,
        };
        return out;
    }
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
        end_locations: vec![],
    };
}

fn next_location(loc: Location, dir: Direction) -> Location {
    return match dir {
        Direction::Right => Location {
            x: loc.x + 1,
            y: loc.y,
        },
        Direction::Left => Location {
            x: loc.x - 1,
            y: loc.y,
        },
        Direction::Up => Location {
            x: loc.x,
            y: loc.y - 1,
        },
        Direction::Down => Location {
            x: loc.x,
            y: loc.y + 1,
        },
    };
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

enum Status {
    Done,
    Pass,
    Bounds,
}

fn test_location(loc: Location, curr_value: u32, name: &str, grid: &mut Grid) -> Status {
    match grid.get_value(loc) {
        Some(test) => {
            // println!("{name} {:?} {test}", loc);
            if test == curr_value + 1 {
                if test == 9 {
                    // println!("Done {test}");
                    grid.end_locations.push(loc);
                    return Status::Done;
                } else {
                    // println!("Pass {test}");
                    return Status::Pass;
                }
            }
            return Status::Bounds;
        }
        None => return Status::Bounds,
    }
}

fn search_trail(loc: Location, curr_value: u32, grid: &mut Grid) -> u32 {
    let mut result = 0;
    let left = next_location(loc, Direction::Left);
    let right = next_location(loc, Direction::Right);
    let up = next_location(loc, Direction::Up);
    let down = next_location(loc, Direction::Down);

    match test_location(left, curr_value, "Left ", grid) {
        Status::Done => result += 1,
        Status::Pass => result += search_trail(left, curr_value + 1, grid),
        Status::Bounds => {}
    }

    match test_location(right, curr_value, "Right", grid) {
        Status::Done => result += 1,
        Status::Pass => result += search_trail(right, curr_value + 1, grid),
        Status::Bounds => {}
    }

    match test_location(up, curr_value, "Up", grid) {
        Status::Done => result += 1,
        Status::Pass => result += search_trail(up, curr_value + 1, grid),
        Status::Bounds => {}
    }

    match test_location(down, curr_value, "Down", grid) {
        Status::Done => result += 1,
        Status::Pass => result += search_trail(down, curr_value + 1, grid),
        Status::Bounds => {}
    }

    // match grid.get_value(left) {
    //     Some(test) => {
    //         println!("Left  {:?} {test}", left);
    //         if test == curr_value + 1 {
    //             if test == 9 {
    //                 println!("Done {test}");
    //                 grid.end_locations.push(left);
    //                 result += 1;
    //             } else {
    //                 println!("Pass {test}");
    //                 result += search_trail(left, test, grid);
    //             }
    //         }
    //     }
    //     None => {}
    // }
    // match grid.get_value(right) {
    //     Some(test) => {
    //         println!("Right {:?} {test}", right);

    //         if test == curr_value + 1 {
    //             if test == 9 {
    //                 println!("Done {test}");
    //                 grid.end_locations.push(right);
    //                 result += 1;
    //             } else {
    //                 println!("Pass {test}");
    //                 result += search_trail(right, test, grid);
    //             }
    //         }
    //     }
    //     None => {
    //         println!("Out of bounds {:?}", up);
    //     }
    // }
    // match grid.get_value(up) {
    //     Some(test) => {
    //         println!("Up    {:?} {test}", up);

    //         if test == curr_value + 1 {
    //             if test == 9 {
    //                 println!("Done {test}");
    //                 grid.end_locations.push(right);
    //                 result += 1;
    //             } else {
    //                 println!("Pass {test}");
    //                 result += search_trail(up, test, grid);
    //             }
    //         }
    //     }
    //     None => {
    //         println!("Out of bounds {:?}", up);
    //     }
    // }
    // match grid.get_value(down) {
    //     Some(test) => {
    //         println!("Down  {:?} {test}", down);

    //         if test == curr_value + 1 {
    //             if test == 9 {
    //                 println!("Done {test}");
    //                 grid.end_locations.push(right);
    //                 result += 1;
    //             } else {
    //                 println!("Pass {test}");
    //                 result += search_trail(down, test, grid);
    //             }
    //         }
    //     }
    //     None => {
    //         println!("Out of bounds {:?}", down);
    //     }
    // }
    return result;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid = make_empty_grid(1, 1);

    let mut start_locations: Vec<Location> = vec![];

    for (l, line) in lines.enumerate() {
        // println!("{}", line);
        let mut row: Vec<u32> = vec![];
        for (c, test) in line.chars().enumerate() {
            row.push(test.to_digit(10).unwrap());
            if test == '0' {
                start_locations.push(Location {
                    x: c as isize,
                    y: l as isize,
                });
            }
        }
        grid.rows.push(row);
    }

    print_rows(&grid.rows);
    println!("Start {:?}", start_locations);

    grid.height = grid.rows.len() as isize;
    grid.width = grid.rows[0].len() as isize;
    grid.end_locations.clear();

    let mut trails = 0;

    for loc in start_locations {
        // let loc = start_locations[0];
        let trailhead = search_trail(loc, 0, &mut grid);

        // println!("Trails: {trails}, {:?}", grid.end_locations);
        // println!("Trails: {}", grid.end_locations.len());

        let unique: Vec<&Location> = grid.end_locations.iter().unique().collect();

        // println!("Unique: {:?}, {:?}", unique.len(), unique);
        println!("Unique: {}", unique.len());

        // trails += unique.len();
        trails += trailhead;
        grid.end_locations.clear();
    }
    // println!("Total: {trails}, {:?}", grid.end_locations);
    println!("Total: {trails}");

    return Ok(());
}
