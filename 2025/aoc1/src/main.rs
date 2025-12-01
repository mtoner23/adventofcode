use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();
    let mut dial: i32 = 50;
    let mut count = 0;

    for l in lines {
        println!("{}", l);
        let mut chars = l.chars();
        let direction;
        if chars.next().unwrap() == 'L' {
            direction = false;
        } else {
            direction = true;
        }

        let value_orig = chars.collect::<String>().parse::<i32>().unwrap();

        let mut dial_orig = dial;
        let mut value = value_orig;
        loop {
            // had to go the pedantic while loop route b/c my modulo math kept hitting edge cases
            if direction {
                if dial + value > 99 {
                    count += 1;
                    if value > 99 {
                        value -= 100
                    } else {
                        dial = (dial + value).rem_euclid(100);
                        break;
                    }
                } else {
                    dial += value;
                    break;
                }
            } else {
                if dial - value < 0 {
                    if value > 99 {
                        count += 1;
                        value -= 100;
                    } else {
                        if dial != 0 {
                            // This really threw a wrench in my thought process
                            count += 1;
                        }
                        dial = (dial - value).rem_euclid(100);
                        break;
                    }
                } else {
                    dial -= value;
                    if dial == 0 {
                        count += 1;
                        // println!("ZERO");
                    }
                    break;
                }
            }
            // println!("Dial {}, Count: {}", dial, count);
        }
        println!("Final Dial {}, Count: {}", dial, count);
        if direction {
            dial_orig += value_orig;
        } else {
            dial_orig -= value_orig;
        }

        if dial_orig.rem_euclid(100) != dial {
            println!(
                "ERROR! final dial != calculated value. {} != {}",
                dial,
                dial_orig.rem_euclid(100)
            );
        }
    }

    println!("Count: {}", count);

    return Ok(());
}
