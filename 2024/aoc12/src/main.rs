use std::error::Error;
use std::fs;
use std::hash::Hash;

type Rows = Vec<Vec<char>>;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    rows: Rows,
    visiteds: Vec<Vec<bool>>,
    height: isize,
    width: isize,
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

    fn get_value(&self, loc: Location) -> Option<char> {
        let out = match self.check_bounds(&loc) {
            true => Some(self.rows[loc.y as usize][loc.x as usize]),
            false => None,
        };
        return out;
    }

    fn make_empty_grid(height: isize, width: isize) -> Grid {
        return Grid {
            rows: vec![],
            visiteds: vec![],
            height: height,
            width: width,
        };
    }

    fn print_rows(&self) {
        for r in &self.rows {
            for c in r {
                print!("{c}");
            }
            println!("");
        }
        println!("");
    }

    fn is_visited(&self, loc: &Location) -> bool {
        return self.visiteds[loc.y as usize][loc.x as usize];
    }
}

fn search_region(loc: Location, loc_char: char, grid: &mut Grid) -> (u32, u32) {
    let mut area = 1;
    let mut perim = 0;

    if grid.is_visited(&loc) {
        return (0, 0);
    } else {
        grid.visiteds[loc.y as usize][loc.x as usize] = true;
    }

    let left = next_location(loc, Direction::Left);
    let right = next_location(loc, Direction::Right);
    let up = next_location(loc, Direction::Up);
    let down = next_location(loc, Direction::Down);

    if let Some(left_test) = grid.get_value(left) {
        if left_test != loc_char {
            perim += 1;
        } else {
            let result = search_region(left, left_test, grid);
            area += result.0;
            perim += result.1;
        }
    } else {
        perim += 1;
    }

    if let Some(right_test) = grid.get_value(right) {
        if right_test != loc_char {
            perim += 1;
        } else {
            let result = search_region(right, right_test, grid);
            area += result.0;
            perim += result.1;
        }
    } else {
        perim += 1;
    }
    if let Some(up_test) = grid.get_value(up) {
        if up_test != loc_char {
            perim += 1;
        } else {
            let result = search_region(up, up_test, grid);
            area += result.0;
            perim += result.1;
        }
    } else {
        perim += 1;
    }
    if let Some(down_test) = grid.get_value(down) {
        if down_test != loc_char {
            perim += 1;
        } else {
            let result = search_region(down, down_test, grid);
            area += result.0;
            perim += result.1;
        }
    } else {
        perim += 1;
    }

    return (area, perim);
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

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid = Grid::make_empty_grid(1, 1);

    for (_l, line) in lines.enumerate() {
        let mut row: Vec<char> = vec![];
        for (_c, test) in line.chars().enumerate() {
            row.push(test);
        }
        let visited_row: Vec<bool> = vec![false; row.len()];
        grid.rows.push(row);
        grid.visiteds.push(visited_row);
    }

    // grid.print_rows();

    grid.height = grid.rows.len() as isize;
    grid.width = grid.rows[0].len() as isize;

    let mut result = 0;

    for (y, row) in grid.rows.clone().iter().enumerate() {
        for (x, &test) in row.iter().enumerate() {
            let loc = Location {
                x: x as isize,
                y: y as isize,
            };
            if grid.is_visited(&loc) {
                continue;
            }

            let (area, perim) = search_region(loc, test, &mut grid);

            println!("Region {test}: Area {area} Perim {perim}");
            result += area * perim;
        }
    }

    println!("result {result}");
    return Ok(());
}
