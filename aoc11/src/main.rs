use std::error::Error;
use std::fs;

const GALAXY: char = '#';
const SPACE: char = '.';

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Location {
    x: usize,
    y: usize,
    x_plus: usize,
    y_plus: usize,
}

const MULT: usize = 1000000;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut galaxies: Vec<Location> = vec![];
    let mut column_view: Vec<Vec<char>> = vec![];

    let height = file.lines().count();
    let width = file.lines().next().unwrap().chars().count();

    let mut row_expands: usize = 0;
    let mut col_expands: usize = 0;

    let mut sample_col: Vec<char> = vec![];
    sample_col.resize(height, '0');
    column_view.resize(width, sample_col);

    // dbg!(column_view);

    for (l, line) in lines.enumerate() {
        println!("{}", line);
        let mut empty = true;
        for (c, test) in line.chars().enumerate() {
            column_view[c][l] = test;
            if test == GALAXY {
                empty = false;
                galaxies.push(Location {
                    x: c,
                    y: l,
                    x_plus: 0,
                    y_plus: row_expands,
                })
            }
        }
        if empty {
            // println!("Empty row {l}");
            row_expands += (MULT - 1);
        }
    }

    // dbg!(&galaxies);

    for (c, col) in column_view.iter().enumerate() {
        println!("{:?}", col);
        let mut empty = true;
        for (r, test) in col.iter().enumerate() {
            if *test == GALAXY {
                empty = false;
                let search = Location {
                    x: c,
                    y: r,
                    x_plus: 0,
                    y_plus: 0,
                };
                for gal in galaxies.iter_mut() {
                    if gal.x == search.x && gal.y == search.y {
                        // dbg!(&gal);
                        gal.x_plus += col_expands;
                        // dbg!(&gal);
                    }
                }
            }
        }
        if empty {
            col_expands += (MULT - 1);
        }
    }

    println!("extra rows: {}, extra cols {}", row_expands, col_expands);

    dbg!(&galaxies);

    let mut dists: Vec<usize> = vec![];

    let mut outer_iter = galaxies.iter();
    let mut inner_iter = galaxies.iter();

    while let Some(outer) = outer_iter.next() {
        inner_iter = outer_iter.clone();
        while let Some(inner) = inner_iter.next() {
            // println!("out: {:?}, in: {:?}", outer, inner);
            let x_diff = (outer.x + outer.x_plus).abs_diff(inner.x + inner.x_plus);
            let y_diff = (outer.y + outer.y_plus).abs_diff(inner.y + inner.y_plus);
            let d = x_diff + y_diff;
            // dbg!(d);
            dists.push(d);
        }
    }

    println!("Sum: {}", dists.iter().sum::<usize>());

    return Ok(());
}
