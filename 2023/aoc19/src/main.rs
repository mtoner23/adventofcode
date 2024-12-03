use regex::Regex;
use std::error::Error;
use std::{fs, vec};

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    name: String,
    rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    fn split(&self, value: usize, dir: bool) -> (Self, Self) {
        if dir {
            (
                Range::new(self.start, value),
                Range::new(value + 1, self.end),
            )
        } else {
            (
                Range::new(value, self.end),
                Range::new(self.start, value - 1),
            )
        }
    }

    fn len(&self) -> usize {
        return self.end - self.start + 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn split(&self, value: usize, dir: bool, letter: &str) -> (Self, Self) {
        match letter {
            "x" => {
                let ranges = self.x.split(value, dir);
                let mut left = self.clone();
                let mut right = self.clone();
                left.x = ranges.0;
                right.x = ranges.1;
                (left, right)
            }
            "m" => {
                let ranges = self.m.split(value, dir);
                let mut left = self.clone();
                let mut right = self.clone();
                left.m = ranges.0;
                right.m = ranges.1;
                (left, right)
            }
            "a" => {
                let ranges = self.a.split(value, dir);
                let mut left = self.clone();
                let mut right = self.clone();
                left.a = ranges.0;
                right.a = ranges.1;
                (left, right)
            }
            "s" => {
                let ranges = self.s.split(value, dir);
                let mut left = self.clone();
                let mut right = self.clone();
                left.s = ranges.0;
                right.s = ranges.1;
                (left, right)
            }
            _ => panic!(),
        }
    }

    fn power(&self) -> usize {
        return self.x.len() * self.m.len() * self.a.len() * self.s.len();
    }
}

// fn traverse_rule(insts: &)  -> Vec<PartRange>{

// }

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();

    let inst_regex = Regex::new(r"(?<inst>\w+)\{(?<rules>.+)\}").unwrap();
    let _obj_rejex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    while let Some(l) = lines.next() {
        // println!("{}", l);

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

    let start_range = Range {
        start: 1,
        end: 4000,
    };

    let mut all_parts: Vec<(PartRange, String)> = vec![(
        PartRange {
            x: start_range.clone(),
            m: start_range.clone(),
            a: start_range.clone(),
            s: start_range.clone(),
        },
        "in".to_string(),
    )];

    let mut accept_parts: Vec<PartRange> = vec![];

    while let Some((mut part, name)) = all_parts.pop() {
        println!("Part: {:?}, Name: {:?}", part, name);
        let start = instructions.iter().find(|x| x.name == name).unwrap();
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

                let range = match letter {
                    "x" => &part.x,
                    "m" => &part.m,
                    "a" => &part.a,
                    "s" => &part.s,
                    _ => panic!(),
                };

                // println!("{rule}, range {:?}", range);
                if (dir && (range.start > value)) || (!dir && (range.end < value)) {
                    // println!("Entire range passes  rule");

                    if dest == "A" {
                        accept_parts.push(part.clone());
                    } else if dest == "R" {
                        // println!("Rejected whole, range {:?}", range);
                    } else {
                        // start = instructions.iter().find(|x| x.name == dest).unwrap();
                        all_parts.push((part.clone(), dest.to_string()));
                        // println!("next all_parts {}", start.name);
                    }
                } else if (dir && (range.end < value)) || (!dir && (range.start > value)) {
                    println!("Entire range not in rule");
                } else {
                    let (failed, passed) = part.split(value, dir, letter);
                    // println!("Range split!\nPass {:?},\nFail {:?}", passed, failed);
                    if dest == "A" {
                        accept_parts.push(passed.clone());
                    } else if dest == "R" {
                        // println!("Rejected, range {:?}", range);
                    } else {
                        all_parts.push((passed, dest.to_string()));
                    }
                    part = failed;
                }
            } else if rule == "A" {
                // println!("Accepted, no rule");
                accept_parts.push(part.clone());
            } else if rule == "R" {
                // println!("Rejected, no rule");
            } else {
                // println!("Next {}, no rule", rule);
                all_parts.push((part.clone(), rule.to_string()));
            }
        }
    }

    let mut sum: usize = 0;
    for p in accept_parts {
        // println!("Accepted: {:?}", p);

        sum += p.power();
    }

    println!("Sum: {}", sum);

    return Ok(());
}
