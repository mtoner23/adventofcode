use regex::Regex;
use std::fs;

struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

const GOALS: Balls = Balls {
    red: 12,
    green: 13,
    blue: 14,
};

fn handle_capture(cap: Option<regex::Captures>) -> u32 {
    match cap {
        Some(i) => i["num"].parse().unwrap(),
        None => 0,
    }
}

fn main() {
    let filepath = "src/input.txt";
    println!("in file {} ", filepath);

    let file = fs::read_to_string(filepath);

    // let mut nums: Vec<u16> = vec![];

    let game_regex = Regex::new(r"Game (?<num>\d+):").unwrap();
    let red_regex = Regex::new(r"(?<num>\d+) red").unwrap();
    let blue_regex = Regex::new(r"(?<num>\d+) blue").unwrap();
    let green_regex = Regex::new(r"(?<num>\d+) green").unwrap();

    let mut count: u32 = 0;

    for line in file.unwrap().lines() {
        // println!("Line: {line}");

        let Some(captures) = game_regex.captures(line) else {
            println!("err no game num: {line}");
            return;
        };

        let game_num = &captures["num"];

        let rounds = line.split(":").nth(1).unwrap().split(";");

        let mut balls_game = Balls {
            red: 0,
            blue: 0,
            green: 0,
        };

        println!("{game_num}");
        for round in rounds {
            // println!("{round}");
            // let colors = round.split(",");
            let red = handle_capture(red_regex.captures(round));
            let blue = handle_capture(blue_regex.captures(round));
            let green = handle_capture(green_regex.captures(round));

            let balls_round = Balls {
                red: red,
                blue: blue,
                green: green,
            };

            println!(
                "red: {}, blue: {}, green: {}",
                balls_round.red, balls_round.blue, balls_round.green
            );

            if balls_round.red > balls_game.red {
                balls_game.red = balls_round.red;
            }
            if balls_round.blue > balls_game.blue {
                balls_game.blue = balls_round.blue;
            }
            if balls_round.green > balls_game.green {
                balls_game.green = balls_round.green;
            }
        }

        println!(
            "Game: red: {}, blue: {}, green: {}",
            balls_game.red, balls_game.blue, balls_game.green
        );

        let game_power = balls_game.red * balls_game.green * balls_game.blue;

        count += game_power;

        // if balls_game.red <= GOALS.red
        //     && balls_game.blue <= GOALS.blue
        //     && balls_game.green <= GOALS.green
        // {
        //     let num = game_num.parse::<u32>().unwrap();
        //     count += num;
        //     println!("Num: {num}");
        //     println!("good game: {count}");
        // }
    }

    println!("Count: {count}");
}
