use std::error::Error;
use std::{fs, vec};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Grid {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

// const PIPES: [char; 6] = ['|', '-', 'L', 'J', '7', 'F'];

pub trait Pipe {
    // fn next_pipe(&self, input: &Location) -> Location;
    fn initial_pipe(&self, input: &Location) -> Result<Vec<Location>, &str>;
    fn get_char(&self, loc: &Location) -> char;
    fn connects(&self, s_loc: &Location, n_loc: &Location) -> Result<bool, &str>;
    fn next_pipe(&self, input: &Location, prev: &Location) -> Location;
}

impl Pipe for Grid {
    fn get_char(&self, loc: &Location) -> char {
        return self.map[loc.y][loc.x];
    }
    fn next_pipe(&self, input: &Location, prev: &Location) -> Location {
        let test = self.get_char(input);
        let mut outputs: Vec<Location> = vec![];

        if test == '|' {
            outputs.push(Location {
                x: input.x,
                y: input.y - 1,
            });
            outputs.push(Location {
                x: input.x,
                y: input.y + 1,
            });
        } else if test == '-' {
            outputs.push(Location {
                x: input.x - 1,
                y: input.y,
            });
            outputs.push(Location {
                x: input.x + 1,
                y: input.y,
            });
        } else if test == 'L' {
            outputs.push(Location {
                x: input.x,
                y: input.y - 1,
            });
            outputs.push(Location {
                x: input.x + 1,
                y: input.y,
            });
        } else if test == 'J' {
            outputs.push(Location {
                x: input.x,
                y: input.y - 1,
            });
            outputs.push(Location {
                x: input.x - 1,
                y: input.y,
            });
        } else if test == '7' {
            outputs.push(Location {
                x: input.x - 1,
                y: input.y,
            });
            outputs.push(Location {
                x: input.x,
                y: input.y + 1,
            });
        } else if test == 'F' {
            outputs.push(Location {
                x: input.x + 1,
                y: input.y,
            });
            outputs.push(Location {
                x: input.x,
                y: input.y + 1,
            });
        }

        if outputs[0] == *prev {
            return outputs[1];
        } else if outputs[1] == *prev {
            return outputs[0];
        } else {
            dbg!(outputs);
            panic!();
        }
    }

    fn connects(&self, s_loc: &Location, n_loc: &Location) -> Result<bool, &str> {
        let test = self.get_char(n_loc);
        if n_loc.y > s_loc.y {
            if test == '|' || test == 'L' || test == 'J' || test == 'S' {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if n_loc.y < s_loc.y {
            if test == '|' || test == '7' || test == 'F' || test == 'S' {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if n_loc.x > s_loc.x {
            if test == '-' || test == '7' || test == 'J' || test == 'S' {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else if n_loc.x < s_loc.x {
            if test == '-' || test == 'L' || test == 'F' || test == 'S' {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            println!("connects, bad input. s: {:?} n: {:?}", s_loc, n_loc);
            return Err("Err");
        }
    }
    fn initial_pipe(&self, input: &Location) -> Result<Vec<Location>, &str> {
        let mut tests: Vec<Location> = vec![];
        let mut outputs: Vec<Location> = vec![];
        if input.x > 0 {
            tests.push(Location {
                x: input.x - 1,
                y: input.y,
            });
        }
        if input.y > 0 {
            tests.push(Location {
                x: input.x,
                y: input.y - 1,
            });
        }
        if input.x < self.width - 1 {
            tests.push(Location {
                x: input.x + 1,
                y: input.y,
            });
        }
        if input.y < self.height - 1 {
            tests.push(Location {
                x: input.x,
                y: input.y + 1,
            });
        }
        // dbg!(&tests);
        for test in tests {
            if let Ok(b) = self.connects(input, &test) {
                if b {
                    outputs.push(test.clone());
                }
            } else {
                return Err("Bad input");
            }
        }
        if outputs.len() == 2 {
            return Ok(outputs);
        } else {
            println!("Err: wrong next pipes");
            for o in &outputs {
                println!("{:?}, {}", o, self.get_char(o));
            }
            return Err("Wrong pipes");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut grid: Grid = Grid {
        map: vec![],
        height: lines.clone().count(),
        width: lines.clone().next().unwrap().chars().count(),
    };
    let mut start_loc = Location { x: 0, y: 0 };

    for (l, line) in lines.enumerate() {
        println!("{}", line);
        let mut row: Vec<char> = vec![];
        for (c, char) in line.chars().enumerate() {
            if char == 'S' {
                start_loc = Location { x: c, y: l };
            }
            row.push(char);
        }
        grid.map.push(row);
    }

    let starts = grid.initial_pipe(&start_loc).unwrap();
    let mut left = starts[0];
    let mut right = starts[1];
    let mut prev_left = start_loc;
    let mut prev_right = start_loc;
    let mut count = 1;

    println!(
        "C: {}, L: {:?} {}, R: {:?} {}",
        count,
        left,
        grid.get_char(&left),
        right,
        grid.get_char(&right)
    );
    loop {
        let next_left = grid.next_pipe(&left, &prev_left);
        prev_left = left;
        left = next_left;

        let next_right = grid.next_pipe(&right, &prev_right);
        prev_right = right;
        right = next_right;

        count += 1;

        println!(
            "C: {}, L: {:?} {}, R: {:?} {}",
            count,
            left,
            grid.get_char(&left),
            right,
            grid.get_char(&right)
        );

        if left == right {
            break;
        }

        // if count > 10 {
        //     break;
        // }
    }

    return Ok(());
}
