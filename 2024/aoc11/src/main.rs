// #![feature(f128)]

use count_digits::CountDigits;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

type StoneMap = HashMap<u64, u64>;

fn insert_into_map(map: &mut StoneMap, value: u64, count: u64) {
    match map.get_mut(&value) {
        Some(v) => *v += count,
        None => {
            map.insert(value, count);
            ()
        }
    }
}

fn map_len(map: &StoneMap) -> u64 {
    let mut result: u64 = 0;
    for (_k, v) in map {
        result += v;
    }
    return result;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();
    // let mut stones: Vec<Stone> = vec![];
    let mut stones: StoneMap = HashMap::new();

    for l in lines {
        println!("{}", l);
        let stones_str = l.split_ascii_whitespace();

        for stone in stones_str {
            let value = stone.parse::<u64>().unwrap();
            stones.insert(value, 1);
        }
    }

    let num_iterations = 75;

    for i in 0..num_iterations {
        let mut j = 0;
        let mut new_map: StoneMap = HashMap::new();

        for (&value, count) in stones.iter_mut() {
            // let stone: Stone = stones[j].clone();

            let num_digits: u32 = value.count_digits() as u32;
            if value == 0 {
                insert_into_map(&mut new_map, 1, *count);
            } else if num_digits % 2 == 0 {
                let divisor = 10u64.pow(num_digits / 2);
                let left = value / divisor;
                let right = value % divisor;
                // println!("left {}, right {}, divisor {}", left, right, divisor);
                insert_into_map(&mut new_map, left, *count);
                insert_into_map(&mut new_map, right, *count);
            } else {
                insert_into_map(&mut new_map, value * 2024, *count);
            }
            j += 1;
        }

        stones = new_map;
        // println!("{:?}", stones);

        // let mut stoneMap: HashMap<u64, u64> = HashMap::new();
        // for stone in &stones {
        //     match stoneMap.get_mut(&stone.value) {
        //         Some(count) => *count += stone.count,
        //         None => {
        //             stoneMap.insert(stone.count, stone.count);
        //             ()
        //         }
        //     }
        //     println!("{:?}", stoneMap);
        // }
        println!("Iter {}, size {:?}", i, map_len(&stones));
        // println!("{:?}", stoneMap);
    }

    println!("{:?}", map_len(&stones));

    return Ok(());
}
