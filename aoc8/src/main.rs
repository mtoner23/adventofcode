use regex::Regex;
use std::error::Error;
use std::{fs, vec};

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    return a;
}

fn find_node(target: &String, nodes: &Vec<Node>) -> Node {
    for n in nodes {
        if n.id == *target {
            return n.clone();
        }
    }
    println!("Err");
    return nodes[0].clone();
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let mut lines = file.lines();

    let instructions = lines.next().unwrap();
    lines.next();

    println!("{}", instructions);

    let my_regex =
        Regex::new(r"(?<id>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)")
            .unwrap();

    let mut all_nodes: Vec<Node> = vec![];
    for line in lines {
        // split1 = line.split("=");
        // id = split1.next().unwrap();
        let Some(captures) = my_regex.captures(line) else {
            println!("Err can not parse: {line}");
            return Err(Box::new(core::fmt::Error));
        };
        let N = Node {
            id: captures["id"].to_string(),
            left: captures["left"].to_string(),
            right: captures["right"].to_string(),
        };
        // println!("{:?}", N);
        all_nodes.push(N);
    }

    let mut start_nodes: Vec<Node> = vec![];

    for node in &all_nodes {
        if node.id.ends_with("A") {
            println!("Start: {}", node.id);
            start_nodes.push(node.clone());
        }
    }

    // let mut myNode: Node = find_node(&target_node, &all_nodes);

    let mut counts: Vec<usize> = vec![];
    for myNode in start_nodes.iter_mut() {
        let mut inst = instructions.chars();
        let mut count = 0;
        loop {
            count += 1;
            // println!("{}", i.unwrap());
            let mut i = inst.next();
            if i.is_none() {
                inst = instructions.chars();
                i = inst.next();
            }

            let target_node = if i.unwrap() == 'L' {
                myNode.left.clone()
            } else {
                myNode.right.clone()
            };
            // println!("Tar: {}", target_node);

            if target_node.ends_with("Z") {
                println!("Found a Z!");
                break;
            } else {
                // println!("No Z!");
            }

            *myNode = find_node(&target_node, &all_nodes);

            if myNode.id != target_node {
                println!("ERROR Kill Me");
                return Err(Box::new(core::fmt::Error));
            }
            // if count > 10 {
            //     return Err(Box::new(core::fmt::Error));
            // }
        }
        counts.push(count);
    }

    // let ans = counts.iter().fold(1, lcm);
    let mut ans = 1;
    for c in counts {
        ans = lcm(ans, c);
    }
    dbg!(ans);

    return Ok(());
}
