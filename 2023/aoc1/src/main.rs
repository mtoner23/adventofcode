use std::fs;

const NUMBER_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number(line: &str, reverse: bool) -> Option<u8> {
    if !reverse {
        for i in 0..line.len() {
            let char = line.chars().nth(i).unwrap();
            if char.is_numeric() {
                return Some(char.to_digit(10).unwrap() as u8);
            }
            for (index, num) in NUMBER_STRINGS.iter().enumerate() {
                let num_len = num.len();
                if i + num.len() > line.len() {
                    continue;
                }
                let slice = &line[i..i + num_len];
                if slice == *num {
                    println!("{slice}");
                    return Some(index as u8);
                }
            }
        }
    } else {
        for i in (0..line.len()).rev() {
            let char = line.chars().nth(i).unwrap();
            if char.is_numeric() {
                return Some(char.to_digit(10).unwrap() as u8);
            }
            for (index, num) in NUMBER_STRINGS.iter().enumerate() {
                let num_len = num.len();
                if i < num.len() {
                    continue;
                }
                let slice = &line[(i - num_len + 1)..(i + 1)];
                if slice == *num {
                    println!("{i} {slice}");
                    return Some(index as u8);
                }
            }
        }
    }
    return None;
}
fn main() {
    let filepath = "src/input.txt";
    println!("in file {} ", filepath);

    let file = fs::read_to_string(filepath);

    let mut nums: Vec<u16> = vec![];

    for line in file.unwrap().lines() {
        println!("Line: {line}");
        let first = get_number(&line, false).unwrap();
        let last = get_number(&line, true).unwrap();
        // for char in line.chars() {
        //     if char.is_numeric() {
        //         first = char;
        //         break;
        //     }
        // }

        // println!("first: {first}");
        // println!("last: {last}");

        // for char in line.chars().rev() {
        //     if char.is_numeric() {
        //         last = char;
        //         break;
        //     }
        // }
        // let chars: Vec<char> = vec![first, last];
        // let num: u16 = chars.into_iter().collect::<String>().parse().unwrap();
        // let num:u16 = String::from(first, last).parse().unwrap();
        let num: u16 = first as u16 * 10 + last as u16;
        nums.push(num);
    }

    let mut sum = 0;
    for n in &nums {
        sum += n;
        println!("Num: {n}, Sum: {sum}");
    }
    println!("Sum: {sum}");
}
