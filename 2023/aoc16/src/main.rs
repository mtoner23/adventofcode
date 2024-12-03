use std::error::Error;
use std::{fs, vec};

type Rows = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    rows: Rows,
    visited: Rows,
    height: isize,
    width: isize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

fn print_rows(rows: &Rows) {
    for r in rows {
        for c in r {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn count_visited(rows: &Rows) -> usize {
    let mut count = 0;
    for r in rows {
        for c in r {
            if *c == '#' {
                count += 1;
            }
        }
    }
    return count;
}

fn clear_visited(grid: &mut Grid) {
    grid.visited.clear();
    let mut sample_row = vec![];
    sample_row.resize(grid.width as usize, '.');
    grid.visited.resize(grid.height as usize, sample_row);
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

fn straight(loc: Location, dir: Direction) -> Location {
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

fn laser(start: Location, start_dir: Direction, grid: &mut Grid) {
    let mut loc = start;
    let mut dir = start_dir;
    loop {
        if !check_bounds(loc, dir, grid) {
            // println!("End!");
            return;
        }

        let visited = grid.visited[loc.y as usize][loc.x as usize];
        let test = grid.rows[loc.y as usize][loc.x as usize];

        if visited == '#' && (test == '|' || test == '-') {
            // println!("Split again!");
            return;
        }

        grid.visited[loc.y as usize][loc.x as usize] = '#';

        // println!("Test: {test}");

        if test == '.' {
            loc = straight(loc, dir);
        } else if test == '\\' {
            loc = match dir {
                Direction::Right => Location {
                    x: loc.x,
                    y: loc.y + 1,
                },
                Direction::Left => Location {
                    x: loc.x,
                    y: loc.y - 1,
                },
                Direction::Up => Location {
                    x: loc.x - 1,
                    y: loc.y,
                },
                Direction::Down => Location {
                    x: loc.x + 1,
                    y: loc.y,
                },
            };
            dir = match dir {
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            };
        } else if test == '/' {
            loc = match dir {
                Direction::Right => Location {
                    x: loc.x,
                    y: loc.y - 1,
                },
                Direction::Left => Location {
                    x: loc.x,
                    y: loc.y + 1,
                },
                Direction::Up => Location {
                    x: loc.x + 1,
                    y: loc.y,
                },
                Direction::Down => Location {
                    x: loc.x - 1,
                    y: loc.y,
                },
            };
            dir = match dir {
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            };
        } else if test == '|' {
            match dir {
                Direction::Left | Direction::Right => {
                    // println!("Split!");
                    laser(
                        Location {
                            x: loc.x,
                            y: loc.y - 1,
                        },
                        Direction::Up,
                        grid,
                    );
                    laser(
                        Location {
                            x: loc.x,
                            y: loc.y + 1,
                        },
                        Direction::Down,
                        grid,
                    );
                    return;
                }
                _ => {
                    loc = straight(loc, dir);
                }
            };
        } else if test == '-' {
            match dir {
                Direction::Up | Direction::Down => {
                    // println!("Split!");
                    laser(
                        Location {
                            x: loc.x + 1,
                            y: loc.y,
                        },
                        Direction::Right,
                        grid,
                    );
                    laser(
                        Location {
                            x: loc.x - 1,
                            y: loc.y,
                        },
                        Direction::Left,
                        grid,
                    );
                    return;
                }
                _ => {
                    loc = straight(loc, dir);
                }
            };
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid = Grid {
        rows: vec![],
        visited: vec![],
        height: 0,
        width: 0,
    };

    for (_l, line) in lines.enumerate() {
        // println!("{}", line);
        let mut row: Vec<char> = vec![];
        for (_c, test) in line.chars().enumerate() {
            row.push(test);
        }
        grid.rows.push(row);
    }

    grid.height = grid.rows.len() as isize;
    grid.width = grid.rows[0].len() as isize;

    let mut sample_row = vec![];
    sample_row.resize(grid.width as usize, '.');
    grid.visited.resize(grid.height as usize, sample_row);

    let mut loc = Location { x: 0, y: 0 };
    let mut direction: Direction;
    let mut highest = 0;

    print_rows(&grid.rows);

    for x in 0..grid.width {
        loc.x = x;
        loc.y = 0;
        direction = Direction::Down;
        laser(loc, direction, &mut grid);
        let visited = count_visited(&grid.visited);
        println!("Visit: {}", visited);

        if visited > highest {
            highest = visited;
        }
        clear_visited(&mut grid);
    }

    for x in 0..grid.width {
        loc.x = x;
        loc.y = grid.height - 1;
        direction = Direction::Up;
        laser(loc, direction, &mut grid);
        let visited = count_visited(&grid.visited);
        if visited > highest {
            highest = visited;
        }
        clear_visited(&mut grid);
    }

    for y in 0..grid.height {
        loc.x = 0;
        loc.y = y;
        direction = Direction::Right;
        laser(loc, direction, &mut grid);
        let visited = count_visited(&grid.visited);
        if visited > highest {
            highest = visited;
        }
        clear_visited(&mut grid);
    }

    for y in 0..grid.height {
        loc.x = grid.width - 1;
        loc.y = y;
        direction = Direction::Left;
        laser(loc, direction, &mut grid);
        let visited = count_visited(&grid.visited);
        if visited > highest {
            highest = visited;
        }
        clear_visited(&mut grid);
    }

    // print_rows(&grid.visited);

    println!("Count: {}", highest);

    return Ok(());
}
