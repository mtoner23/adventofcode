use std::error::Error;
use std::fs;

fn level_test(test: &Vec<u32>) -> bool {
    // println!("{:?}", test);

    let mut increasing = true;
    let mut decreasing = true;
    let mut differences = true;

    let mut t_prev: u32 = 0;
    for (idx, &t) in test.iter().enumerate() {
        // println!("{idx}, {t}");
        if idx == 0 {
            t_prev = t;
            continue;
        }

        if t_prev > t {
            increasing = false
        } else if t_prev < t {
            decreasing = false
        } else {
            differences = false;
            // println!("Line {l} Failed, differences");
            break;
        }

        if t_prev.abs_diff(t) > 3 {
            differences = false;
            // println!("Line {l} Failed, differences, {t_prev} {t}");
            break;
        }

        if increasing == false && decreasing == false {
            // println!("Line {l} Failed");
            break;
        } else {
            // println!("pass?");
        }
        t_prev = t;
    }

    if (increasing || decreasing) && differences {
        // println!("Line {l} Passed!");
        return true;
    }
    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut passed = 0;

    for l in lines {
        let split = l.split_ascii_whitespace();
        let mut test: Vec<u32> = vec![];

        for t in split {
            // println!("{t}");
            let elem: u32 = t.to_string().parse::<u32>().unwrap();
            test.push(elem);
        }

        // println!("{:?}", test);
        if level_test(&test) {
            passed += 1;
        } else {
            for idx in 0..test.len() {
                let removed = test.remove(idx);
                if level_test(&test) {
                    passed += 1;
                    break;
                } else {
                    test.insert(idx, removed);
                }
            }
        }
    }

    println!("Passed {passed}");

    return Ok(());
}
