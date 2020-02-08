use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap
};


fn get_input(filename: impl AsRef<Path>) -> HashMap<String, String> {
    let file = File::open(filename).expect("Unable to open file");
    let buf = BufReader::new(file);
    let mut orbits = HashMap::new();

    for line in buf.lines() {
        let orbit = line.expect("Unable to parse line");
        orbits.insert(orbit[4..].to_string(), orbit[..3].to_string());
    }

    orbits
}

fn get_num(sat: &String, body: &String, orbits: &HashMap<String, String>, mut orbit_nums: &mut HashMap<String, u32>) -> u32 {
    if sat == "COM" {
        return 0;
    }

    let mut n = 0;
    if let Some(x) = orbit_nums.get(body) {
        n = x+1;
    } else {
        if let Some(b) = orbits.get(body) {
            let x = get_num(body, b, orbits, &mut orbit_nums);
            n = x+1;
        } else {
            //We're looking at COM here likely which doesn't orbit anything else.
            n += 1;
        }
    }

    orbit_nums.insert(sat.to_string(), n);
    n
}

fn main() {
    let orbits = get_input("resources/input.txt");
    let mut orbit_nums: HashMap<String, u32> = HashMap::new();
    for (sat, body) in &orbits {
        if !orbit_nums.contains_key(sat) {
            let _ = get_num(sat, body, &orbits, &mut orbit_nums);
        }
    } 

    let mut sum = 0;
    for (_, n) in &orbit_nums {
        sum += n;
    }
    println!("Total Orbits: {}", sum);
}
