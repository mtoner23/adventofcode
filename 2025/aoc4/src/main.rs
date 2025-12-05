use std::error::Error;
use std::fs;

// #[derive(Debug, Clone, PartialEq, Copy)]
// pub struct Location {
//     x: usize,
//     y: usize,
// }

#[derive(Debug, Clone)]
pub struct Grid {
    rows: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn check_grid(grid: &Grid, x: usize, y: usize) -> bool {
    let mut count = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if nx >= grid.width || ny >= grid.height {
            continue;
        }
        if grid.rows[ny][nx] == '@' {
            count += 1;
        }
    }
    // println!("x,y = {},{}; count: {}", x, y, count);
    if count < 4 {
        return true;
    } else {
        return false;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let rows: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let height = rows.len();
    let width = rows.get(0).map(|r| r.len()).unwrap_or(0);

    let mut grid = Grid {
        rows,
        height,
        width,
    };

    println!("{:?}", grid);

    let mut total_count = 0;

    loop {
        let mut removed = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.rows[y][x] == '@' {
                    if check_grid(&grid, x, y) {
                        total_count += 1;
                        removed += 1;
                        println!("Found at x: {}, y: {}", x, y);
                        grid.rows[y][x] = '.';
                    }
                }
            }
        }

        if removed == 0 {
            break;
        }
        println!("Looped! Removed: {}", removed);
    }
    println!("Total count: {}", total_count);

    return Ok(());
}
