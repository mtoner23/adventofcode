use std::collections::HashMap;
use std::error::Error;
use std::{fs, vec};

const START: &str = "fft";
const END: &str = "dac";

// should find fft around ~800,000
// total 152973158478561669
// svr fft dac out
// svr -> fft = 2733
// fft -> dac = 4199340
// dac -> out = 25644

#[derive(Clone, Debug, PartialEq)]
struct Label {
    name: String,
    idx: usize,
    outs: Vec<Option<usize>>,
    out_names: Vec<String>,
}

fn print_queue(queue: &Vec<(Label, Vec<String>)>) {
    println!("Queue Size {}", queue.len());
    for l in queue {
        println!("{:?}", l.0);
    }
}

fn visit_label(idx: usize, labels: &Vec<Label>, map: &mut HashMap<String, Option<i64>>) -> i64 {
    let l = labels[idx].clone();
    // let map_lookup = map.get(&l.name).unwrap();
    let mut count = 0;
    for i in 0..l.outs.len() {
        let n_idx = l.outs[i];
        let name = l.out_names[i].clone();
        match n_idx {
            Some(n) => {
                let dest = labels[n].name.clone();
                let dest_lookup = map.get(&dest).unwrap();
                count += match dest_lookup {
                    Some(i) => *i,
                    None => {
                        println!("loop {}", n);
                        visit_label(n, labels, map)
                    }
                }
            }
            // None => count += 1,
            None => {
                if name == END {
                    count += 1
                }
            }
        }
    }
    map.insert(l.name, Some(count));
    return count;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut labels: Vec<Label> = vec![];

    for (i, l) in lines.enumerate() {
        println!("{}", l);
        let mut split = l.split(":");
        let name = split.next().unwrap();
        let outs: Vec<String> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        // println!("Lab: {}, outs {:?}", name, outs);
        labels.push(Label {
            name: name.to_string(),
            idx: i,
            out_names: outs,
            outs: vec![],
        });
    }

    let mut start: Label = Label {
        name: "".to_string(),
        idx: 0,
        outs: vec![],
        out_names: vec![],
    };

    let mut map: HashMap<String, Option<i64>> = HashMap::new();

    for i in 0..labels.len() {
        // let l = &labels[i];
        for o in labels[i].out_names.clone() {
            if o == "out" || o == END {
                labels[i].outs.push(None);
            } else {
                // println!("Searching for {}", o);
                let f = labels.iter().find(|f| f.name == o).unwrap().idx;
                labels[i].outs.push(Some(f));
            }
        }
        // println!("{:?}", labels[i]);

        if labels[i].name == START {
            start = labels[i].clone();
        }

        map.insert(labels[i].name.clone(), None);
    }

    let mut queue: Vec<(Label, Vec<String>)> = vec![];

    let mut next: (Label, Vec<String>) = (start.clone(), vec![]);

    // let mut this_path: Vec<String> = vec![];
    // let mut i = 0;
    // let mut start_time = std::time::Instant::now();

    let count = visit_label(start.idx, &labels, &mut map);
    println!("count {}", count);

    // loop {
    //     let next_idxs = next.0.outs.clone();
    //     let mut path = next.1.clone();

    //     let mut discovered = true;
    //     let mut count = 0;
    //     print!("Start {}, ", next.0.name);
    //     for n in next_idxs {
    //         match n {
    //             Some(i) => {
    //                 let name = labels[i].name.clone();
    //                 let get = map.get(&name).unwrap();

    //                 print!("Label {}, ", name);

    //                 if get.is_none() {
    //                     print!("Unmapped , ");
    //                     discovered = false;
    //                     path.push(name);
    //                     queue.push((labels[i].clone(), path.clone()));
    //                 } else {
    //                     print!("Allmapped, ");
    //                     // count += get.unwrap();
    //                     map.insert(next.0.name.clone(), get.unwrap());
    //                 }
    //             }
    //             None => {
    //                 count += 1;
    //             }
    //         }
    //     }
    //     if discovered {
    //         println!("discov true");
    //         // map.insert(next.0.name, Some(count));
    //     } else {
    //         println!("discov false");
    //         // map.insert(next.0.name, Some(count));
    //     }

    //     // print_queue(&queue);
    //     // println!("Map: {:?}", map);
    //     match queue.pop() {
    //         Some(n) => next = n,
    //         None => break,
    //     };
    //     i += 1;
    //     if i % 1000 == 0 {
    //         let elapsed = start_time.elapsed();
    //         println!(
    //             "iter {}, path size {}: - elapsed: {:?}",
    //             i,
    //             path.len(),
    //             elapsed
    //         );
    //         start_time = std::time::Instant::now();
    //         // break;
    //     }
    // }
    // println!("Map: {:?}", map);
    // println!("Count: {:?}", map.get(START).unwrap());

    return Ok(());
}
