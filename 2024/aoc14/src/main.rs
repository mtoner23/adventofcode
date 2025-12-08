use regex::Regex;
use std::error::Error;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn add(&self, other: Coord) -> Coord {
        return Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

    fn mult(&self, val: i32) -> Coord {
        return Coord {
            x: self.x * val,
            y: self.y * val,
        };
    }

    fn wrap(&self, height: i32, width: i32) -> Coord {
        let mut new = Coord {
            x: self.x % width,
            y: self.y % height,
        };

        if new.x < 0 {
            new.x += width;
        }

        if new.y < 0 {
            new.y += height;
        }
        return new;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Robot {
    p: Coord,
    v: Coord,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut robots: Vec<Robot> = vec![];

    let robot_regex = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    for line in lines {
        println!("{}", line);
        let Some((full, [p_x, p_y, v_x, v_y])) =
            robot_regex.captures(line).map(|caps| caps.extract())
        else {
            return Err("Ahh".into());
        };
        // println!("{}, {}, {}, {}", p_x, p_y, v_x, v_y);
        let p: Coord = Coord {
            x: p_x.parse().unwrap(),
            y: p_y.parse().unwrap(),
        };

        let v: Coord = Coord {
            x: v_x.parse().unwrap(),
            y: v_y.parse().unwrap(),
        };
        let robot = Robot { p: p, v: v };
        println!("robot {:?}", robot);
        robots.push(robot);
    }

    let num_iter: i32 = 100;
    let height: i32 = 11;
    let width: i32 = 11;

    for robot in robots.iter_mut() {
        robot.p = robot.p.add(robot.v.mult(num_iter)).wrap(height, width);
        println!("{:?}", robot);
    }

    return Ok(());
}
