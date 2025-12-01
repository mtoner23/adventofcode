use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test2.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();
    let mut dial: i32 = 50;
    let mut count = 0;

    for l in lines {
        println!("{}", l);
        let mut chars = l.chars();
        let mut direction = true;
        if chars.next().unwrap() == 'L' {
            // print!("Left ");
            direction = false;
        } else {
            // print!("Right ");
            direction = true;
        }

        let mut value = chars.collect::<String>().parse::<i32>().unwrap();
        while true{
        if direction {
            if dial + value > 99 {
                count += 1
                if value > 100{
                    value = value - 100
                }else{
                    break;
                }
            }
            dial += value % 100;
        } else {
            dial -= value;
        }
    }

        // if dial > 99 {
        //     count += dial / 100;
        //     println!("Dial: {}, Count: {}", dial.rem_euclid(100), count);
        //     dial = dial.rem_euclid(100);
        // } else if dial < 0 {
        //     count += dial / 100;
        //     if dial % 100 != 0 {
        //         count += 1;
        //     }
        //     println!("Dial: {}, Count: {}", dial.rem_euclid(100), count);
        //     dial = dial.rem_euclid(100);
        // } else {
        //     println!("Dial {}, Count: {}", dial, count);
        // }

        // if dial == 0 {
        //     count += 1;
        //     println!("Count++");
        // }
    }

    println!("Count: {}", count);

    return Ok(());
}
