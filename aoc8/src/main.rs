use regex::Regex;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn find_node(target: String, nodes: &Vec<Node>) -> Node {
    for n in nodes {
        if n.id == target {
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
        Regex::new(r"(?<id>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();

    let mut nodes: Vec<Node> = vec![];
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
        nodes.push(N);
    }

    let mut target_node: String = "AAA".to_string();
    let mut myNode: Node = find_node(target_node, &nodes);
    let mut inst = instructions.chars();
    let mut count = 0;
    while true {
        count += 1;
        let mut i = inst.next();
        if i.is_none() {
            inst = instructions.chars();
            i = inst.next();
        }

        // println!("{}", i.unwrap());

        if i.unwrap() == 'L' {
            target_node = myNode.left.clone();
        } else {
            target_node = myNode.right.clone();
        }

        // println!("{}", target_node);

        if target_node == "ZZZ" {
            println!("Done! {}", count);
            return Ok(());
        }

        for n in &nodes {
            if n.id == target_node {
                myNode = n.clone();
                break;
            }
        }

        if myNode.id != target_node {
            println!("ERROR Kill");
            break;
        }
    }

    return Err(Box::new(core::fmt::Error));
}
