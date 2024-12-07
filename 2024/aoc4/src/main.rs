use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut letters: Vec<Vec<char>> = vec![];

    for line in lines {
        let vec_line = line.chars().collect();
        letters.push(vec_line);
        // println!("{line}");
    }
    let mut first_result = 0;
    let mut second_result = 0;
    for (l, &ref line) in letters.iter().enumerate() {
        for (c, &curr_c) in line.iter().enumerate() {
            if curr_c == 'X' {
                first_result += search_for_xmas(l, c, &letters);
            }

            if curr_c == 'A' {
                second_result += search_for_cross(l, c, &letters);
            }
        }
    }

    println!("Found first: {first_result}, second: {second_result}");
    return Ok(());
}

fn search_for_cross(l: usize, c: usize, letters: &Vec<Vec<char>>) -> usize {
    let l_size = letters.len();
    let c_size = letters[0].len();

    if !check_bounds(l as i32 - 1, c as i32 - 1, l_size, c_size) {
        return 0;
    }
    if !check_bounds(l as i32 - 1, c as i32 + 1, l_size, c_size) {
        return 0;
    }
    if !check_bounds(l as i32 + 1, c as i32 + 1, l_size, c_size) {
        return 0;
    }
    if !check_bounds(l as i32 + 1, c as i32 - 1, l_size, c_size) {
        return 0;
    }

    let up_left = letters[l - 1][c - 1];
    let up_right = letters[l - 1][c + 1];
    let down_left = letters[l + 1][c - 1];
    let down_right = letters[l + 1][c + 1];

    let mut cross = 0;

    if up_left == 'M' && down_right == 'S' {
        // println!("Coord {l},{c}: ");
        cross += 1
    }
    if up_left == 'S' && down_right == 'M' {
        cross += 1
    }

    if up_right == 'M' && down_left == 'S' {
        cross += 1
    }
    if up_right == 'S' && down_left == 'M' {
        cross += 1
    }

    if cross == 2 {
        return 1;
    } else {
        return 0;
    }
}

fn search_for_xmas(l: usize, c: usize, letters: &Vec<Vec<char>>) -> usize {
    let xmas = ['X', 'M', 'A', 'S'];
    let l_size = letters.len();
    let c_size = letters[0].len();
    let mut found_count = 0;
    let l_int = l as i32;
    let c_int = c as i32;
    if letters[l][c] == xmas[0] {
        let mut found = true;

        // Up
        for i in 1..4 {
            if !check_bounds(l_int - i as i32, c_int, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l - i][c] != xmas[i] {
                found = false;
                break;
            }
            // print!("{letters}")
        }
        if found {
            // println!("Coord {l},{c}: Found Up");
            found_count += 1;
        }
        found = true;

        // Down
        for i in 1..4 {
            if !check_bounds(l_int + i as i32, c_int, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l + i][c] != xmas[i] {
                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Down");
            found_count += 1;
        }
        found = true;

        // Left
        for i in 1..4 {
            if !check_bounds(l_int, c_int - i as i32, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l][c - i] != xmas[i] {
                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Left");
            found_count += 1;
        }
        found = true;

        // Right
        for i in 1..4 {
            if !check_bounds(l_int, c_int + i as i32, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l][c + i] != xmas[i] {
                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Right");
            found_count += 1;
        }
        found = true;

        // Up Left
        for i in 1..4 {
            if !check_bounds(l_int - i as i32, c_int - i as i32, l_size, c_size) {
                if l == 5 && c == 6 {
                    // println!("Check Bounds!,{l},{c},{i}");
                }
                found = false;
                break;
            }
            if letters[l - i][c - i] != xmas[i] {
                if l == 5 && c == 6 {
                    let letter = letters[l - i][c - i];
                    // println!("Letters!, {l},{c},{i}: {letter}");
                }

                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Up/Left");
            found_count += 1;
        }
        found = true;

        // Down Left
        for i in 1..4 {
            if !check_bounds(l_int + i as i32, c_int - i as i32, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l + i][c - i] != xmas[i] {
                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Down/Left");
            found_count += 1;
        }
        found = true;

        // Up Right
        for i in 1..4 {
            if !check_bounds(l_int - i as i32, c_int + i as i32, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l - i][c + i] != xmas[i] {
                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Up/Right");
            found_count += 1;
        }
        found = true;

        // Down Right
        for i in 1..4 {
            if !check_bounds(l_int + i as i32, c_int + i as i32, l_size, c_size) {
                found = false;
                break;
            }
            if letters[l + i][c + i] != xmas[i] {
                found = false;
                break;
            }
        }
        if found {
            // println!("Coord {l},{c}: Found Down/Right");
            found_count += 1;
        }
        found = true;
    }
    return found_count;
}

fn check_bounds(l: i32, c: i32, l_size: usize, c_size: usize) -> bool {
    if l < 0 {
        return false;
    }
    if c < 0 {
        return false;
    }
    if l >= l_size as i32 {
        return false;
    }
    if c >= c_size as i32 {
        return false;
    }
    true
}
