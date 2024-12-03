use std::error::Error;
use std::fs;

fn get_hash(input: &str) -> usize {
    let mut curr = 0;
    for letter in input.chars() {
        let ascii: usize = letter as usize;
        curr += ascii;
        curr *= 17;
        curr = curr % 256;
    }
    return curr;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();
    let line = lines.next().unwrap();
    let seqs = line.split(",");

    let mut boxes: Vec<Vec<(&str, &str)>> = vec![];
    boxes.resize(256, vec![]);

    for seq in seqs {
        if seq.contains("=") {
            let mut split = seq.split("=");
            let label = split.next().unwrap();
            let number = split.next().unwrap();
            let hash = get_hash(label);
            let the_box: &mut Vec<(&str, &str)> = boxes.get_mut(hash).unwrap();
            let mut found = false;
            for b in the_box.iter_mut() {
                if b.0 == label {
                    b.1 = number;
                    found = true;
                }
            }
            if !found {
                the_box.push((label, number));
            }
        } else if seq.contains("-") {
            let mut split = seq.split("-");
            let label = split.next().unwrap();
            let hash = get_hash(label);
            let the_box: &mut Vec<(&str, &str)> = boxes.get_mut(hash).unwrap();

            for i in 0..the_box.len() {
                if the_box[i].0 == label {
                    the_box.remove(i);
                    break;
                }
            }
        } else {
            println!("Undecodeable seq: {}", seq);
            panic!();
        }
    }
    let mut sum = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        for (slot, (_label, focal)) in b.iter().enumerate() {
            // dbg!(&slot, &_label, &focal);
            let power = (box_num + 1) * (slot + 1) * focal.parse::<usize>().unwrap();
            sum += power;
            println!("Power: {power}");
            // sum += get_sum(&b);
        }
    }
    println!("Sum: {}", sum);

    return Ok(());
}
