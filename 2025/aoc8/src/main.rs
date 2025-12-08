use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    points: Vec<usize>,
    // size: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Distance {
    p1: usize, // indeces into master points array
    p2: usize,
    distance: f32,
}

fn get_distance(p1: &Point, p2: &Point) -> f32 {
    let dx = (p1.x - p2.x) as f32;
    let dy = (p1.y - p2.y) as f32;
    let dz = (p1.z - p2.z) as f32;

    return (dx * dx + dy * dy + dz * dz).sqrt();
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();
    let mut points: Vec<Point> = Vec::new();
    let mut circuits: Vec<Circuit> = Vec::new();

    for l in lines {
        // println!("{}", l);
        let split = l
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let point = Point {
            x: split[0],
            y: split[1],
            z: split[2],
        };
        points.push(point);
        // println!("{:?}", point);
    }

    for p in 0..points.len() {
        circuits.push(Circuit {
            points: vec![p],
            // size: 1,
        });
    }

    let mut distancses: Vec<Distance> = Vec::new();

    for p in 0..points.len() {
        for q in p + 1..points.len() {
            let d = get_distance(&points[p], &points[q]);
            let distance = Distance {
                p1: p,
                p2: q,
                distance: d,
            };
            distancses.push(distance);
        }
    }

    distancses.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // println!("Distances: {:?}", distancses);
    // let num_connect = 1000;
    let mut num_iter = 0;
    loop {
        let d = &distancses[num_iter];
        // println!("Connecting {}, {:?} ", num_iter, d);

        let c1 = circuits
            .iter_mut()
            .position(|x| x.points.contains(&d.p1))
            .unwrap();
        let c2 = circuits
            .iter()
            .position(|x| x.points.contains(&d.p2))
            .unwrap();

        if c1 != c2 {
            let mut appending = circuits[c2].points.clone();
            circuits[c1].points.append(&mut appending);
            circuits.remove(c2);
        }

        if num_iter % 100 == 0 {
            println!("Num Iter {}", num_iter)
        }

        if circuits.len() == 1 {
            let p1 = points[d.p1];
            let p2 = points[d.p2];
            let answer = p1.x * p2.x;
            println!("p1 {:?}, p2 {:?}, answer {}", p1, p2, answer);
            break;
        }
        num_iter += 1;

        // println!("Circuits after {}, {:?}", i, circuits);
    }

    //Part 1 results
    // circuits.sort_by(|a, b| b.points.len().cmp(&a.points.len()));
    // println!("Circuits Final: {:?}", circuits);
    // let total = circuits[0].points.len() * circuits[1].points.len() * circuits[2].points.len();
    // println!("Total: {}", total);

    return Ok(());
}
