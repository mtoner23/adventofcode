use std::error::Error;
use std::fs;

fn try_operators(answer: usize, nums: &Vec<usize>, result: usize) -> bool {
    if nums.len() == 0 {
        return answer == result;
    }
    if result > answer {
        return false;
    }
    let trimmed = nums[1..].to_vec();
    if try_operators(answer, &trimmed, result * nums[0]) {
        return true;
    }

    if try_operators(answer, &trimmed, result + nums[0]) {
        return true;
    }

    // let next = (result.to_string() + &nums[0].to_string())
    //     .parse::<usize>()
    //     .unwrap();

    let length = &nums[0].checked_ilog10().unwrap_or(0) + 1;
    let next = result * 10usize.pow(length) + &nums[0];

    if try_operators(answer, &trimmed, next) {
        return true;
    }

    return false;
}
fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut result = 0;

    for l in lines {
        let mut splits = l.split(':');
        let answer = splits.next().unwrap().parse::<usize>().unwrap();
        let ops = splits.next().unwrap().split_ascii_whitespace();
        let mut nums = vec![];
        for o in ops {
            nums.push(o.parse::<usize>().unwrap());
        }
        if try_operators(answer, &nums[1..].to_vec(), nums[0]) {
            result += answer;
        }
        // println!("{} : {:?}", answer, nums);
    }

    println!("Result: {result}");

    return Ok(());
}
