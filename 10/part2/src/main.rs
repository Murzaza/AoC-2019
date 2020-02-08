use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap
};

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not get lines") )
        .collect()    
}

fn get_slope(x: i32, y: i32, xi: i32, yi: i32) -> f64 {
    (y-yi) as f64 / (x-xi) as f64
}

fn get_man_dist(x: i32, y:i32, xi: i32, yi: i32) -> i32 {
    ((x-xi) + (y-yi)).abs()
}

fn get_quad(x: i32, xi: i32) -> usize {
    if ((xi-x) / ((xi-x).abs())) > 0 {
        0
    } else {
        1
    }
}

fn get_num(seen_asteroids: &HashMap<String, [i32;2]>) -> u32 {
    let mut sum = 0;
    for (k, v) in seen_asteroids.iter() {
        if v[0] != -1i32 {
            //println!("{} 0 {}", k, v[0]);
            sum += 1;
        }
        if v[1] != -1i32 {
            //println!("{} 1 {}", k, v[1]);
            sum += 1;
        }
    }

    sum
}

fn scan(asteroids: &Vec<String>, x: usize, y: usize) -> u32 {
    let mut visible_asteroids: HashMap<String, [i32;2]> = HashMap::new();
    for yi in 0..asteroids.len() {
        for xi in 0..asteroids[yi].len() {
            if asteroids[yi].as_bytes()[xi] as char == '#' && (x!=xi && y!=yi) {
                let slope = match xi == x {
                    true => String::from("vert"),
                    false => get_slope(x as i32,y as i32,xi as i32,yi as i32).to_string()
                };
                let dist = get_man_dist(x as i32,y as i32,xi as i32,yi as i32);
                let quad = match slope.as_ref() {
                    "vert" => get_quad(y as i32, yi as i32),
                    _ => get_quad(x as i32, xi as i32)
                };

                //Print out the found values
                if slope.eq("2") && dist == 3 && quad == 1 {
                    println!( "{}, {}", xi, yi);
                }
                match visible_asteroids.get_mut(&slope) {
                    Some(data) => {
                        if data[quad] == -1i32 || data[quad] > dist {
                            data[quad] = dist;
                        }
                    },
                    None => {
                        let mut data = [-1i32, -1i32];
                        data[quad as usize] = dist;
                        visible_asteroids.insert(slope, data);
                    }
                };
            }
        }
    }

    get_num(&visible_asteroids)
}

fn get_station(asteroids: Vec<String>) -> (usize, usize, u32) {
    let mut max_asteroids = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for y in 0..asteroids.len() {
        for x in 0..asteroids[y].len() {
            if asteroids[y].as_bytes()[x] as char == '#' {
                let ast = scan(&asteroids, x, y);
                if ast > max_asteroids {
                    max_asteroids = ast;
                    max_x = x;
                    max_y = y;
                } 
            }
        }
    }

    (max_x, max_y, max_asteroids)
}

fn main() {
    let asteroids = get_input("resources/input.txt");
    //let (x, y, ast) = get_station(asteroids);
    //println!("Best location for a station: {}, {}: {} asteroids", x, y, ast);
    let ast = scan(&asteroids, 20,21);
    println!("ast: {}", ast);
}
