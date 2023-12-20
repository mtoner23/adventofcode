use regex::Regex;
use std::error::Error;
use std::{fs, vec};

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    name: String,
    rules: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/test.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();

    let inst_regex = Regex::new(r"(?<inst>\w+)\{(?<rules>.+)\}").unwrap();
    let obj_rejex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    while let Some(l) = lines.next() {
        println!("{}", l);

        if l.len() == 0 {
            break;
        }

        let Some(captures) = inst_regex.captures(l) else {
            println!("Err can not regex: {l}");
            return Err(Box::new(core::fmt::Error));
        };

        let name = &captures["inst"];
        let rules = &captures["rules"];
        println!("inst: {name}, rules {rules}");

        let mut split = rules.split(",");
        let mut rules_vec: Vec<String> = vec![];

        while let Some(r) = split.next() {
            rules_vec.push(r.to_string());
        }

        let inst = Instruction {
            name: name.to_string(),
            rules: rules_vec,
        };

        instructions.push(inst);
    }

    let mut count: u64 = 0;

    let mut x = vec![Range {
        start: 1,
        end: 4000,
    }];
    let mut m = x.clone();
    let mut a = x.clone();
    let mut s = x.clone();

    // for x in 1..4000 {
    //     for m in 1..4000 {
    //         for a in 1..4000 {
    //             for s in 1..4000 {
    loop {
        // while let Some(l) = lines.next() {
        //     let Some(captures) = obj_rejex.captures(l) else {
        //         println!("Err can not regex: {l}");
        //         return Err(Box::new(core::fmt::Error));
        //     };
        //     let (x, m, a, s) = (
        //         &captures[1].parse::<usize>().unwrap(),
        //         &captures[2].parse::<usize>().unwrap(),
        //         &captures[3].parse::<usize>().unwrap(),
        //         &captures[4].parse::<usize>().unwrap(),
        //     );

        // println!("x {:?}, m {:?}, a {:?}, s {:?}", x, m, a, s);

        let mut start = instructions.iter().find(|x| x.name == "in").unwrap();

        loop {
            let mut end = false;
            for rule in &start.rules {
                if rule.contains(":") {
                    let mut split = rule.split(":");
                    let comp = split.next().unwrap();
                    let dest = split.next().unwrap();
                    let (mut split_comp, dir) = if comp.contains(">") {
                        (comp.split(">"), true)
                    } else {
                        (comp.split("<"), false)
                    };

                    let letter = split_comp.next().unwrap();
                    let value = split_comp.next().unwrap().parse::<usize>()?;

                    // println!("{letter} {} {value}:{dest}", (if dir { ">" } else { "<" }));

                    let decision;
                    if dir {
                        let ranges = match letter {
                            "x" => &mut x,
                            "m" => &mut m,
                            "a" => &mut a,
                            "s" => &mut s,
                            _ => panic!(),
                        };

                        while let Some(r) = ranges.pop() {
                            
                        }
                    } else {
                        decision = match letter {
                            "x" => x < value,
                            "m" => m < value,
                            "a" => a < value,
                            "s" => s < value,
                            _ => panic!(),
                        };
                    }
                    if decision {
                        if dest == "A" {
                            // println!("Accepted");
                            count += 1;
                            end = true;
                        } else if dest == "R" {
                            // println!("Rejected");
                            end = true;
                        } else {
                            start = instructions.iter().find(|x| x.name == dest).unwrap();
                            // println!("next dest {}", start.name);
                        }
                        break;
                    }
                } else if rule == "A" {
                    // println!("Accepted");
                    count += 1;
                    end = true;
                    break;
                } else if rule == "R" {
                    // println!("Rejected");
                    end = true;
                    break;
                } else {
                    start = instructions
                        .iter()
                        .find(|x| x.name == rule.as_str())
                        .unwrap();
                    // println!("next dest {}", start.name);
                }
            }
            if end {
                break;
            }
            // break;
        }
    }
    println!("Sum: {}", count);

    return Ok(());
}
