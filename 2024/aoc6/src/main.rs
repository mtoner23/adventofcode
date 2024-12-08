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

fn check_bounds(loc: Location, dir: Direction, grid: &Grid) -> bool {
    // println!("loc : {:?}, dir: {:?}", loc, dir);
    return !match dir {
        Direction::Up => loc.y < 0,
        Direction::Down => loc.y >= grid.height,
        Direction::Left => loc.x < 0,
        Direction::Right => loc.x >= grid.width,
    };
}

fn rotate_dir(dir: Direction) -> Direction {
    return match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
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

fn find_loop(start_location: Location, start_dir: Direction, grid: &Grid) -> bool {
    let mut curr_location = start_location;
    let mut curr_dir = start_dir;
    let mut count = 0;
    let mut visiteds: Vec<(Location, Direction)> = vec![];

    // visiteds.push((start_location,start_dir));
    loop {
        let mut next_loc = next_location(curr_location, curr_dir);
        if !check_bounds(next_loc, curr_dir, &grid) {
            println!("Escaped, no loop");
            return false;
        }

        let test = grid.rows[next_loc.y as usize][next_loc.x as usize];

        if test == '#' || test == 'O' {
            curr_dir = rotate_dir(curr_dir);
            next_loc = next_location(curr_location, curr_dir);
            let test2 = grid.rows[next_loc.y as usize][next_loc.x as usize];
            if test2 == '#' || test2 == 'O' {
                curr_dir = rotate_dir(curr_dir);
                next_loc = next_location(curr_location, curr_dir);
            }

            let found = visiteds.iter().find(|&&x| x == (curr_location, curr_dir));
            if found.is_some() {
                println!("Found Loop!!");
                // print_rows(&grid.rows);
                return true;
            }
            visiteds.push((curr_location, curr_dir));
        }

        // println!("Step {:?}, {:?}", next_loc, curr_dir);
        curr_location = next_loc;

        count += 1;

        if count == 100000 {
            println!("Timeout!! {:?}, {:?}", curr_location, curr_dir);
            print_rows(&grid.rows);
            return false;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid = make_empty_grid(0, 0);

    let mut start_location: Location = Location { x: 0, y: 0 };

    for (l, line) in lines.enumerate() {
        // println!("{}", line);
        let mut row: Vec<char> = vec![];
        for (c, test) in line.chars().enumerate() {
            row.push(test);
            if test == '^' {
                start_location = Location {
                    x: c as isize,
                    y: l as isize,
                };
            }
        }
        grid.rows.push(row);
    }

    // print_rows(&grid.rows);
    println!("Start {:?}", start_location);

    grid.height = grid.rows.len() as isize;
    grid.width = grid.rows[0].len() as isize;

    let mut curr_location = start_location;
    let mut curr_dir = Direction::Up;
    let mut count = 0;

    let mut visiteds: Vec<Location> = vec![];

    loop {
        let mut next_loc = next_location(curr_location, curr_dir);
        if !check_bounds(next_loc, curr_dir, &grid) {
            // grid.rows[curr_location.y as usize][curr_location.x as usize] = 'X';
            visiteds.push(curr_location);
            break;
        }

        let test = grid.rows[next_loc.y as usize][next_loc.x as usize];

        if test == '#' {
            curr_dir = rotate_dir(curr_dir);
            next_loc = next_location(curr_location, curr_dir);
        }

        // grid.rows[curr_location.y as usize][curr_location.x as usize] = 'X';
        let found = visiteds.iter().find(|&&x| x == curr_location);
        if found.is_none() {
            visiteds.push(curr_location);
        }

        curr_location = next_loc;

        count += 1;

        if count == 100000 {
            println!("Timeout!! {:?}, {:?}", curr_location, curr_dir);
            break;
        }
    }

    // let mut result = 0;
    let mut solutions = vec![];

    for visit in visiteds {
        if visit == start_location {
            continue;
        }

        let old_char = grid.rows[visit.y as usize][visit.x as usize];

        grid.rows[visit.y as usize][visit.x as usize] = 'O';
        // print_rows(&grid.rows);
        if find_loop(start_location, Direction::Up, &grid) {
            solutions.push(visit);
        }
        grid.rows[visit.y as usize][visit.x as usize] = old_char;
    }

    println!("Result: {}", solutions.len());
    return Ok(());
}
