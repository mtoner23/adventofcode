use std::error::Error;
use std::{fs, vec};

type Line = Vec<char>;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Grid {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

fn compare_lines(start: &Line, next: &Line) -> usize {
    let mut diffs = 0;
    assert_eq!(start.len(), next.len());
    for (s, n) in start.iter().zip(next.iter()) {
        if s != n {
            diffs += 1;
        }
    }
    return diffs;
}

fn test_mirror(start: usize, next: usize, lines: &Vec<Line>) -> usize {
    if next >= lines.len() {
        return 0;
    } else {
        let count = compare_lines(&lines[start], &lines[next]);
        // println!("s: {start}, n: {next}, count: {count}");
        if count <= 1 {
            if start == 0 || (next + 1) == lines.len() {
                return count;
            } else {
                return count + test_mirror(start - 1, next + 1, lines);
            }
        } else {
            return count;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines().peekable();
    let mut sum: usize = 0;
    // let mut line_offset: usize = 0;

    while lines.peek() != None {
        let mut grid: Grid = Grid {
            rows: vec![],
            columns: vec![],
            height: 0,
            width: 0,
        };
        while let Some(line) = lines.next() {
            println!("{}", line);
            let mut row: Vec<char> = vec![];
            if line.chars().count() == 0 {
                println!("Empty");
                break;
            }
            for (_c, char) in line.chars().enumerate() {
                row.push(char);
            }
            grid.width = row.len();
            grid.rows.push(row);
        }

        grid.height = grid.rows.len();

        let mut sample_col: Vec<char> = vec![];
        sample_col.resize(grid.height, '0');
        grid.columns.resize(grid.width, sample_col);

        for (c, col) in grid.columns.iter_mut().enumerate() {
            for (r, row) in col.iter_mut().enumerate() {
                *row = grid.rows[r][c];
            }
        }

        // dbg!(&grid);
        let mut mirrors: Vec<usize> = vec![];

        println!("Rows");
        for row in 0..grid.rows.len() - 1 {
            let mirror = test_mirror(row, row + 1, &grid.rows);
            if mirror == 1 {
                println!("Mirror! {mirror}");
                mirrors.push((row + 1) * 100);
                break;
            }
        }

        println!();

        println!("Columns");
        for col in 0..grid.columns.len() - 1 {
            // dbg!(&grid.columns);
            let mirror = test_mirror(col, col + 1, &grid.columns);
            if mirror == 1 {
                println!("Mirror! {mirror}");
                mirrors.push(col + 1);
                break;
            }
        }

        if mirrors.len() == 0 {
            panic!();
        }

        dbg!(&mirrors);

        sum += mirrors.iter().sum::<usize>();
    }

    println!("Sum: {}", sum);

    return Ok(());
}
