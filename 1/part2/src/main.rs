use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get_fuel_required(n: i64) -> i64 {
    let x = n / 3;
    x - 2
}

fn get_input(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|line| line.expect("Could not parse line").parse::<i64>().expect("Could not convert string to i64"))
       .collect()
}

fn main() {
    let mut sum = 0;

    let inputs = get_input("resources/input.txt");
    for input in inputs {
        let mut fuel = get_fuel_required(input);
        while fuel > 0 {
            sum += fuel;
            fuel = get_fuel_required(fuel);
        }
    }

    println!("{}", sum);
}
