use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

#[derive(Debug)]
struct Ops {
    direction: String,
    value: i32
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    vertical: bool
}

fn get_input(filename: impl AsRef<Path>) -> Vec<Vec<Ops>> {
    let file = File::open(filename).expect("Could not open file");
    let buf = BufReader::new(file);

    buf.lines()
       .map(|line| {
            let line = line.expect("Could not parse line");
            let mut ops = Vec::new();
            for op in line.as_str().split(',') {
                let direction = String::from(&op[0..1]);
                let value = op[1..].parse::<i32>().expect("Unable to convert to i32");
                ops.push(Ops {direction, value});
            }
            ops
       })
       .collect()
}

fn main() {
    let inputs = get_input("resources/input.txt");

    let mut points = [Vec::new(), Vec::new()];
    points[0].push(Point {x: 0, y: 0, vertical: false});
    points[1].push(Point {x: 0, y: 0, vertical: false});

    let mut wire_counter = 0;
    for wire in inputs {
        for segment in wire {
            let mut last = points[wire_counter].last().expect("No last element...");
            match segment.direction.as_ref() {
                "U" => { points[wire_counter].push( Point {x: last.x, y: last.y + segment.value, vertical: true} ) },
                "D" => { points[wire_counter].push( Point {x: last.x, y: last.y - segment.value, vertical: true} ) },
                "R" => { points[wire_counter].push( Point {x: last.x + segment.value, y: last.y, vertical: false} ) },
                "L" => { points[wire_counter].push( Point {x: last.x - segment.value, y: last.y, vertical: false} ) },
                _ => { println!("LOL WUT") }
            }
        }
        wire_counter += 1;
    }

    let mut intersections = Vec::new();
    //Get the intersections.
    for i in 1..points[0].len() {
        let start_0 = &points[0][i-1];
        let end_0 = &points[0][i];

        for j in 1..points[1].len() {
            let start_1 = &points[1][j-1];
            let end_1 = &points[1][j];

            if end_1.vertical != end_0.vertical {
                match end_0.vertical {
                    true => {
                        if ((start_1.x > end_0.x && end_1.x < end_0.x) ||
                           (start_1.x < end_0.x && end_1.x > end_0.x)) &&
                           ((start_0.y < end_1.y && end_0.y > end_1.y) ||
                           (start_0.y > end_1.y && end_0.y < end_1.y)) {
                               intersections.push( Point {x: end_0.x ,y: end_1.y ,vertical: false})
                           }
                    },
                    false => {
                        if ((start_1.y > end_0.y && end_1.y < end_0.y) ||
                           (start_1.y < end_0.y && end_1.y > end_0.y)) &&
                           ((start_0.x < end_1.x && end_0.x > end_1.x) ||
                           (start_0.x > end_1.x && end_0.x < end_1.x)) {
                               intersections.push( Point {x: end_1.x ,y: end_0.y ,vertical: false})
                           }
                    }
                };
            }
        }
    }

    let mut smallest = 0;
    let mut man_dist = 999999;
    for i in 0..intersections.len() {
        if intersections[i].x.abs() + intersections[i].y.abs() < man_dist {
            man_dist = intersections[i].x.abs() + intersections[i].y.abs();
            smallest = i;
        }
    }

    println!("Smallest intersection is {:?} with {}", intersections[smallest], man_dist);
}
