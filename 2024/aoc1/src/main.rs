use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for l in lines {
        // println!("{l}");
        let mut elems = l.split_ascii_whitespace();
        let elem1 = elems.nth(0).unwrap().parse::<i32>().unwrap();
        // println!("{elem1}");
        let elem2 = elems.nth(0).unwrap().parse::<i32>().unwrap();
        // println!("{elem2}");
        list1.push(elem1);
        list2.push(elem2);
    }

    list1.sort();
    list2.sort();

    assert!(list1.len() == list2.len());

    let mut result = 0;

    for (idx1, elem1) in list1.iter().enumerate() {
        let mut sim_score = 0;
        for (idx2, elem2) in list2.iter().enumerate() {
            // if elem1 > elem2 {
            // break;
            // } else {

            if elem2 == elem1 {
                sim_score += elem1;
            }
        }
        println!("elem1 {elem1}, simScore {sim_score}");
        result += sim_score;
    }

    println!("Result: {result}");

    return Ok(());
}
