use core::panic;
use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    // let re = Regex::new(r"(\[.*\]) \((.*)\) (\{\})").unwrap();
    // capture [.#.] (1,2,3) {4,5,6}
    // \(([^\)]*)\)

    // let re = Regex::new(r"^\[([^\]]*)\].*(\([^)]*\))/g.*\{([^}]*)\}").unwrap();

    let mut total_steps = 0;

    for l in lines {
        println!("{}", l);
        // let m = re.captures(l).unwrap();
        // println!("{:?}", m);
        let mut goals: Vec<bool> = Vec::new();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut joltage: Vec<char> = Vec::new();

        let mut goals_find = false;
        let mut buttons_find = false;
        let mut joltage_find = false;

        let mut buttons_str = String::new();
        for c in l.chars() {
            if c == '[' {
                goals_find = true;
                // println!("Found [");
            } else if c == ']' {
                goals_find = false;
                // println!("Found ], {:?}", goals);
            } else if c == '(' {
                buttons_find = true;
                // println!("Found (");
            } else if c == ')' {
                buttons_find = false;
                let nums: Vec<usize> = buttons_str
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                buttons.push(nums);
                // println!("Found ), {}", buttons_str);
                buttons_str.clear();
            } else if c == '{' {
                //TBD
                // println!("Found {{");
            } else if c == '}' {
                // println!("Found }}");
            } else {
                if goals_find {
                    goals.push(c == '#');
                } else if buttons_find {
                    buttons_str.push(c);
                }
            }
        }
        println!("Goals {:?}, Buttons {:?}", goals, buttons);

        let mut state: Vec<bool> = vec![false; goals.len()];

        let mut found = false;
        let mut steps = 0;
        for size in 0..=buttons.len() {
            for combo in buttons.iter().combinations(size) {
                // println!("{:?}", combo);

                for c in combo {
                    for &b in c {
                        state[b] = !state[b];
                    }
                    if state == goals {
                        println!("Found solution: {:?} in {} steps", c, size);
                        steps = size;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
                state = vec![false; goals.len()];
            }
            if found {
                break;
            }
        }
        if found {
            total_steps += steps;
        } else {
            panic!("Could not find button solution");
        }
    }

    println!("Total steps {}", total_steps);

    return Ok(());
}
