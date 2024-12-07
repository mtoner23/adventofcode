use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut rules: Vec<(usize, usize)> = vec![];

    let mut in_rules = true;
    let mut result = 0;

    for l in lines {
        // println!("{}, len{}", l, l.len());
        if l.len() == 0 {
            in_rules = false;
            println!("{:?}", rules);
        } else if in_rules {
            let mut splits = l.split('|');
            rules.push((
                splits.nth(0).unwrap().parse().unwrap(),
                splits.nth(0).unwrap().parse().unwrap(),
            ))
        } else {
            let updates_str = l.split(',');
            let mut update = vec![];
            for u in updates_str {
                update.push(u.parse::<usize>().unwrap());
            }
            let mut pass = true;
            for (u, &num) in update.iter().enumerate() {
                for r in &rules {
                    if num == r.0 {
                        for earlier in 0..u {
                            if update[earlier] == r.1 {
                                pass = false;
                                break;
                            }
                        }
                    }
                    if !pass {
                        break;
                    }
                }
                if !pass {
                    break;
                }
            }
            if pass {
                assert!(update.len() % 2 == 1);
                let middle = update[update.len() / 2];
                result += middle;
                println!("Passed {l}, middle {middle}");
            }
        }
    }

    println!("result: {result}");

    return Ok(());
}
