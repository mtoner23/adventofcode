#![feature(iter_next_chunk)]

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut filesystem: Vec<i64> = vec![];

    // let empty = -1;

    for l in lines {
        let mut file_id = 0;
        let mut new_l = l.to_string();
        if l.len() % 2 == 1 {
            new_l.push('0');
        }
        let mut chars = new_l.chars();
        while let Ok([length_file, length_free]) = chars.next_chunk() {
            // println!("Len file {}, Len Free {}", length_file, length_free);
            let mut file = vec![file_id; length_file.to_digit(10).unwrap() as usize];
            let mut free = vec![-1; length_free.to_digit(10).unwrap() as usize];
            filesystem.append(&mut file);
            filesystem.append(&mut free);
            file_id += 1;
        }

        let mut i = 0;
        let length = filesystem.len();
        println!("Len {:?}", filesystem.len());
        println!("Beg {:?}", filesystem);

        loop {
            let test = filesystem[i];
            if test == -1 {
                let mut not_free = length - 1;
                let mut not_free = length - 1;
                for &elem in filesystem.iter().rev() {
                    if elem != -1 {
                        file_id = elem;
                    }
                    not_free -= 1;
                }

                if not_free < i {
                    println!("Done! {},", i);
                    break;
                }
                // println!("not free {}", not_free);

                let data = filesystem.remove(not_free);
                // println!("Rem {:?}", filesystem);
                let empty = filesystem.remove(i);
                // println!("Rem {:?}", filesystem);

                filesystem.insert(i, data);
                // println!("Ins {:?}", filesystem);

                filesystem.insert(length - 1, empty);
                // println!("Ins {:?}", filesystem);

                // println!("Removed {:?}, inserted", not_free)
            }
            // println!("{:?}", filesystem);
            i += 1;
        }

        println!("{:?}", filesystem);

        let mut result = 0;
        let mut count = 0;
        for id in filesystem.clone() {
            if id == -1 {
                break;
            }
            result += count * id;
            count += 1;
        }
        println!("{result}");
    }

    return Ok(());
}
