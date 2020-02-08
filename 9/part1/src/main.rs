use std::fs;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, BufRead};

fn get_program(filename: &str) -> Vec<i128> {
    let input = fs::read_to_string(filename).expect("Unable to read file");
    input.as_str().split(',')
        .map( |x| x.parse::<i128>().expect("Couldn't convert string to i128") )
        .collect()
}

fn get_inputs(code: i128, a: i128, b: i128, relative: &i128, inputs: &Vec<i128>, mem: &HashMap<i128, i128>) -> (i128, i128) {
    (get_input(code%10, a, relative, inputs, mem), get_input(code/10, b, relative, inputs, mem))
}

fn get_input(code: i128, val: i128, relative: &i128, inputs: &Vec<i128>, mem: &HashMap<i128, i128>) -> i128 {
    match code {
        0 => get_value(val, inputs, mem),
        1 => val,
        2 => get_value(relative + val, inputs, mem),
        _ => 0 //Bruh
    }
}

fn get_value(val: i128, inputs: &Vec<i128>, mem: &HashMap<i128, i128>) -> i128 {
    if val as usize > inputs.len() {
        match mem.get(&val) {
            Some(x) => *x,
            None => 0
        }
    } else {
        inputs[val as usize]
    }
}

fn set_value(idx: i128, val: i128, inputs: &mut Vec<i128>, mem: &mut HashMap<i128, i128>) {
    if idx as usize > inputs.len() {
        mem.insert(idx, val);
    } else {
        inputs[idx as usize] = val;
    }
}

fn get_dest(code: i128, val: i128, relative: &i128) -> i128 {
    match code == 2 {
        true => *relative + val,
        false => val
    }
}

fn execute(inputs: &mut Vec<i128>) {
    let mut i: usize = 0;
    let mut relative: i128 = 0;
    let mut memory = HashMap::new();

    //Go through the program an execute.
    while i < inputs.len() {
        let op = inputs[i] % 100;
        let code = inputs[i] / 100;
        match op {
            1 => {  //Add
                    let dest = get_dest(code/100, inputs[i+3], &relative);
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(code % 100, a, b, &relative, inputs, &memory);
                    set_value(dest, a_val + b_val, inputs, &mut memory);
                    i += 4;
                 },
            2 => {  //Multiply
                    let dest = get_dest(code/100, inputs[i+3], &relative);
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(code % 100, a, b, &relative, inputs, &memory);
                    set_value(dest, a_val*b_val, inputs, &mut memory);
                    i += 4;
                 },
            3 => {  //Input
                    let dest = get_dest(code, inputs[i+1], &relative);
                    println!("Please enter a number: ");
                    let val = io::stdin().lock().lines().next().unwrap().unwrap().parse::<i128>().expect("Can't convert user input to i128");
                    set_value(dest, val, inputs, &mut memory);
                    i += 2;
                 },
            4 => {  //Output
                    let a = inputs[i+1];
                    let (a_val, _) = get_inputs(code, a, 0, &relative, inputs, &memory);
                    println!("> {}", a_val);
                    i += 2;
                 },
            5 => {  //jump-if-true
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(code, a, b, &relative, inputs, &memory);
                    if a_val != 0 {
                        i = b_val.try_into().unwrap();
                    } else {
                        i += 3;
                    }
                 },
            6 => {  //jump-if-false
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(code, a, b, &relative, inputs, &memory);
                    if a_val == 0 {
                        i = b_val.try_into().unwrap();
                    } else {
                        i += 3;
                    }
                 },
            7 => {  //less than
                    let dest = get_dest(code/100, inputs[i+3], &relative);
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(code % 100, a, b, &relative, inputs, &memory);
                    if a_val < b_val {
                        set_value(dest, 1, inputs, &mut memory);
                    } else {
                        set_value(dest, 0, inputs, &mut memory);
                    }
                    i += 4;
                 },
            8 => {  //equal
                    let dest = get_dest(code/100, inputs[i+3], &relative);
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(code % 100, a, b, &relative, inputs, &memory);
                    if a_val == b_val {
                        set_value(dest, 1, inputs, &mut memory);
                    } else {
                        set_value(dest, 0, inputs, &mut memory);
                    }
                    i += 4;
                 },
            9 => { //relative move
                    let a = inputs[i+1];
                    let (a_val, _) = get_inputs(code, a, 0, &relative, inputs, &memory);
                    relative += a_val;
                    i += 2;
                 },
            99 => { //end
                    return;
                  },
            _ => { //error
                    println!("We shouldn't be here inputs[{}] = {} -> {}", i, inputs[i], inputs[i] % 100);
                    return
                 }
        };
    }
}

fn main() {
    let mut program = get_program("resources/input.txt");
    execute(&mut program);
}