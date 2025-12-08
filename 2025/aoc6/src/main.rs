use core::num;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let num_lines = lines.clone().count() - 1;

    // let mut cols: Vec<Vec<i64>> = vec![];
    let mut cols: Vec<Vec<Vec<char>>> = vec![];
    let mut ops: Vec<char> = vec![];
    let mut results: Vec<i64> = vec![];
    let mut lengths: Vec<usize> = vec![];

    let last_line = lines.clone().last().unwrap();

    let mut count = 0;
    for c in last_line.chars() {
        if c == '+' || c == '*' {
            ops.push(c);
            if count > 0 {
                lengths.push(count - 1);
                let empty: Vec<Vec<char>> = vec![vec![' '; num_lines]; count - 1];
                cols.push(empty);
            }
            count = 1;
        } else {
            count += 1;
        }
    }
    lengths.push(count);
    cols.push(vec![vec![' '; num_lines]; count]);

    println!("lengths: {:?}", lengths);
    // println!("{:?}", cols);

    for (l_idx, l) in lines.enumerate() {
        println!("{}", l);
        let mut idx = 0;
        let mut idx_vec = 0;
        // let mut this_len = lengths[idx_vec]
        if l_idx >= num_lines {
            // println!("Reached operaters!");
            break;
        }
        for c in l.chars() {
            if idx >= lengths[idx_vec] {
                idx = 0;
                idx_vec += 1;
                continue;
                // this_len = lengths[idx_vec];
            }
            // println!(
            //     "assigning cols [{}][{}][{}], c = {}",
            //     idx_vec, idx, l_idx, c
            // );
            cols[idx_vec][idx][l_idx] = c;
            idx += 1;
        }
    }

    println!("{:?}", cols);

    // println!("cols: {:?}", cols);
    // println!("ops: {:?}", ops);

    // results = cols[0].clone();

    // let num: i32 = chars.iter().collect::<String>().parse().unwrap();

    let mut i = 0;
    for col in cols {
        let op = ops[i];
        let mut result: i64 = match op {
            '*' => 1,
            '+' => 0,
            _ => 0,
        };
        for number in col {
            println!("Number {:?}", number);
            let n: i64 = number
                .iter()
                .collect::<String>()
                .chars()
                .filter(|c| *c != ' ')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            println!("n {}", n);
            match op {
                '*' => result *= n,
                '+' => result += n,
                _ => {}
            }
        }
        i += 1;
        results.push(result);
    }

    println!("Results: {:?}", results);

    let final_result: i64 = results.iter().sum();
    println!("final result: {}", final_result);

    return Ok(());
}
