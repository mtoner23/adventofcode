// use petgraph::graph::*;
use core::panic;
use petgraph::graphmap::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

use rustworkx_core::connectivity::stoer_wagner_min_cut as swmc;

// #[derive(Debug, Clone)]
// struct Node {
//     id: String,
//     verts: Vec<String>,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    // let mut graph = StableUnGraph::<String, ()>::default();

    // let mut seen_nodes: Vec<&str> = vec![];

    let mut edges = HashSet::<(&str, &str)>::new();

    for l in lines {
        println!("{}", l);
        let mut split1 = l.split(":");
        let id = split1.next().unwrap();
        // let mut n = Node {
        //     id: id.to_string(),
        //     verts: vec![],
        // };
        let mut split2 = split1.next().unwrap().split_whitespace();
        while let Some(v) = split2.next() {
            edges.insert((id, v));
        }
    }
    let graph = UnGraphMap::<&str, ()>::from_edges(edges);
    dbg!(&graph);

    let Ok(Some((_, group))) = swmc(&graph, |_| Ok::<_, ()>(1)) else {
        panic!()
    };

    dbg!(&group);

    let g1 = group.len();
    let g2 = graph.node_count() - g1;

    let mult = g1 * g2;

    dbg!(mult);

    return Ok(());
}
