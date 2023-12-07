use std::error::Error;
use std::fs;

fn get_distance(start: usize, total: usize) -> usize {
    assert!(start <= total);
    return start * (total - start);
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();

    let times = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace();

    let mut distances = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace();

    let mut total = 1;
    let mut time_string = "".to_owned();
    let mut dist_string = "".to_owned();
    for (time, distance) in times.zip(distances) {
        time_string += time;
        dist_string += distance;
    }
    let distance: usize = dist_string.parse().unwrap();
    let time: usize = time_string.parse().unwrap();
    println!("Time {}, d {}", time, distance);
    let mut count = 0;
    for t in 0..time {
        let d = get_distance(t, time);
        // println!("d -> {}", d);
        if d > distance {
            count += 1;
        }
    }
    println!("Count: {}", count);
    total *= count;
    println!("Total: {}", total);

    return Ok(());
}
