use std::error::Error;
use std::{fs, vec};

fn get_differences(nums: Vec<isize>) -> isize {
    let mut diffs: Vec<isize> = vec![];
    let mut n_list = nums.iter();
    let mut prev = n_list.next().unwrap();
    for n in n_list {
        // assert!(n >= prev);
        let diff = n - prev;
        diffs.push(diff);
        prev = n
    }
    println!("Diffs {:?}", diffs);

    for d in &diffs {
        if *d != 0 {
            return diffs.first().unwrap() - get_differences(diffs.clone());
        }
    }
    return 0;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut predictions: Vec<isize> = vec![];
    for line in lines {
        let chars = line.split_whitespace();
        let mut numbers = vec![];
        for num in chars {
            numbers.push(num.parse::<isize>().unwrap());
        }
        println!("In: {:?}", numbers);
        let pred = numbers.first().unwrap() - get_differences(numbers.clone());
        predictions.push(pred);
        println!("P: {}", pred);
    }

    println!("Predictions {:?}", predictions);
    println!("Sum: {}", predictions.iter().sum::<isize>());

    return Ok(());
}
