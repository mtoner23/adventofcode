use std::error::Error;
use std::{fs, vec};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

fn print_rows(rows: &Vec<Vec<char>>) {
    for r in rows {
        for c in r {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn tilt_north(grid: &mut Grid) {
    let rows = &mut grid.rows;
    for r in 0..grid.height {
        for s in 0..grid.width {
            let symbol = rows[r][s];
            if symbol != 'O' {
                continue;
            }
            let mut position = r;
            while position > 0 {
                let next_symbol = rows[position - 1][s];
                if next_symbol == '.' {
                    rows[position][s] = '.';
                    rows[position - 1][s] = 'O';
                    position -= 1;
                } else if next_symbol == '#' || next_symbol == 'O' {
                    break;
                } else {
                    println!("Reached unknwon symbol!");
                    panic!()
                }
            }
        }
    }
}
fn tilt_south(grid: &mut Grid) {
    let rows = &mut grid.rows;
    // println!("{}", grid.height);
    for r in (0..grid.height).rev() {
        for s in (0..grid.width).rev() {
            let symbol = rows[r][s];
            if symbol != 'O' {
                continue;
            }
            let mut position = r;
            while position < rows.len() - 1 {
                let next_symbol = rows[position + 1][s];
                if next_symbol == '.' {
                    rows[position][s] = '.';
                    rows[position + 1][s] = 'O';
                    position += 1;
                } else if next_symbol == '#' || next_symbol == 'O' {
                    break;
                } else {
                    println!("Reached unknwon symbol!");
                    panic!()
                }
            }
        }
    }
}
fn tilt_west(grid: &mut Grid) {
    let rows = &mut grid.rows;

    for c in 0..grid.width {
        for s in 0..grid.height {
            let symbol = rows[s][c];
            if symbol != 'O' {
                continue;
            }
            let mut position = c;
            while position > 0 {
                let next_symbol = rows[s][position - 1];
                if next_symbol == '.' {
                    rows[s][position] = '.';
                    rows[s][position - 1] = 'O';
                    position -= 1;
                } else if next_symbol == '#' || next_symbol == 'O' {
                    break;
                } else {
                    println!("Reached unknwon symbol!");
                    panic!()
                }
            }
        }
    }
}
fn tilt_east(grid: &mut Grid) {
    let rows = &mut grid.rows;

    for c in (0..grid.width).rev() {
        for s in (0..grid.height).rev() {
            let symbol = rows[s][c];
            if symbol != 'O' {
                continue;
            }
            let mut position = c;
            while position < grid.width - 1 {
                let next_symbol = rows[s][position + 1];
                if next_symbol == '.' {
                    rows[s][position] = '.';
                    rows[s][position + 1] = 'O';
                    position += 1;
                } else if next_symbol == '#' || next_symbol == 'O' {
                    break;
                } else {
                    println!("Reached unknwon symbol!");
                    panic!()
                }
            }
        }
    }
}

const NUM_CYCLES: usize = 1000000000;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid: Grid = Grid {
        rows: vec![],
        cols: vec![],
        height: 0,
        width: 0,
    };

    for (_l, line) in lines.enumerate() {
        println!("{}", line);
        let mut row: Vec<char> = vec![];
        for (_c, test) in line.chars().enumerate() {
            row.push(test);
        }
        grid.rows.push(row);
    }
    grid.height = grid.rows.len();
    grid.width = grid.rows[0].len();

    let mut sample_col: Vec<char> = vec![];
    sample_col.resize(grid.height, '0');
    grid.cols.resize(grid.width, sample_col);

    for (c, col) in grid.cols.iter_mut().enumerate() {
        for (r, row) in col.iter_mut().enumerate() {
            *row = grid.rows[r][c];
        }
    }

    let mut prev_grids: Vec<Grid> = vec![];
    let mut prev_match = 0;
    let mut prev_diff = 0;
    let mut index = 0;

    for cycle in 0..NUM_CYCLES {
        // println!("After {} cycles:", cycle + 1);
        prev_grids.push(grid.clone());
        tilt_north(&mut grid);
        // print_rows(&grid.rows);

        tilt_west(&mut grid);
        // print_rows(&grid.rows);

        tilt_south(&mut grid);
        // print_rows(&grid.rows);

        tilt_east(&mut grid);
        // print_rows(&grid.rows);

        if prev_grids.contains(&grid) {
            let diff = cycle - prev_match;
            println!(
                "MATCH! prev: {}, curr: {}, diff {}",
                prev_match,
                cycle,
                cycle - prev_match
            );
            if diff == prev_diff {
                println!("Loop!: {diff}");
                index = (NUM_CYCLES - cycle - 1) % prev_diff;
                break;
            }
            prev_match = cycle;
            prev_diff = diff;
            prev_grids.clear();
            // break;
        }
    }

    println!("diff {}, index {}", prev_diff, index);
    let mut load = grid.height;
    let mut sum = 0;
    // print_rows(&prev_grids[index].rows);
    for r in &prev_grids[index].rows {
        for c in r {
            if *c == 'O' {
                sum += load;
            }
        }
        load -= 1;
    }
    println!("Sum: {sum}");

    return Ok(());
}
