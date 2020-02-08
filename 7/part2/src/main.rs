use std::fs;
use std::convert::TryInto;
use itertools::Itertools;

fn get_program(filename: &str) -> Vec<i64> {
    let input = fs::read_to_string(filename).expect("Unable to read file");
    input.as_str().split(',')
        .map( |x| x.parse::<i64>().expect("Couldn't convert string to i64") )
        .collect()
}

fn get_inputs(code: i64, a: i64, b: i64, inputs: &Vec<i64>) -> (i64, i64) {
    match code {
        0 => (inputs[a as usize], inputs[b as usize]),
        1 => (a, inputs[b as usize]),
        10 => (inputs[a as usize], b),
        11 => (a, b),
        _ => (0,0) //Bruh
    }
}

fn execute(inputs: &mut Vec<i64>, start: &mut usize, phase: i64, input: &mut i64) -> usize {
    let mut i = (*start).clone();
    let mut first = true;

    //Go through the program an execute.
    while i < inputs.len() {
        match inputs[i] % 100 {
            1 => {  //Add
                    let dest: usize = inputs[i+3].try_into().unwrap();
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(inputs[i]/100, a, b, inputs);
                    inputs[dest] = a_val + b_val;
                    i += 4;
                 },
            2 => {  //Multiply
                    let dest: usize = inputs[i+3].try_into().unwrap();
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(inputs[i]/100, a, b, inputs);
                    inputs[dest] = a_val * b_val;
                    i += 4;
                 },
            3 => {  //Input
                    let dest: usize = inputs[i+1].try_into().unwrap();
                    //println!("Please enter a number: ");
                    //let val = io::stdin().lock().lines().next().unwrap().unwrap().parse::<i64>().expect("Can't convert user input to i64");
                    if first && *start == 0 {
                        inputs[dest] = phase; 
                        first = false;
                    } else {
                        inputs[dest] = *input;
                    }
                    i += 2;
                 },
            4 => {  //Output
                    let a = inputs[i+1];
                    let (a_val, _) = get_inputs(inputs[i]/100, a, 0, inputs);
                    //println!("Output: {}", a_val);
                    *input = a_val;
                    *start = i + 2;
                    return 4;
                 },
            5 => {  //jump-if-true
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(inputs[i]/100, a, b, inputs);
                    if a_val != 0 {
                        i = b_val.try_into().unwrap();
                    } else {
                        i += 3;
                    }
                 },
            6 => {  //jump-if-false
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(inputs[i]/100, a, b, inputs);
                    if a_val == 0 {
                        i = b_val.try_into().unwrap();
                    } else {
                        i += 3;
                    }
                 },
            7 => {  //less than
                    let dest: usize = inputs[i+3].try_into().unwrap();
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(inputs[i]/100, a, b, inputs);
                    if a_val < b_val {
                        inputs[dest] = 1;
                    } else {
                        inputs[dest] = 0;
                    }
                    i += 4;
                 },
            8 => {  //equal
                    let dest: usize = inputs[i+3].try_into().unwrap();
                    let a = inputs[i+1];
                    let b = inputs[i+2];
                    let (a_val, b_val) = get_inputs(inputs[i]/100, a, b, inputs);
                    if a_val == b_val {
                        inputs[dest] = 1;
                    } else {
                        inputs[dest] = 0;
                    }
                    i += 4;
                 },
            99 => {
                    *start = i;
                    return 99;
                  },
            _ => {
                    println!("We shouldn't be here inputs[{}] = {} -> {}", i, inputs[i], inputs[i] % 100);
                    return 99;
                 }
        };
    }

    println!("We really shouldn't be here!");
    return 99;
}

fn main() {
    let program = get_program("resources/input.txt");
    let mut max_out = 0;

    for amps in (5..10).permutations(5) {
        //program, next instruction, phase
        let mut programs = vec![program.clone(), program.clone(), program.clone(), program.clone(), program.clone()];
        let mut indices: Vec<usize> = vec![0,0,0,0,0];
        let mut code: usize = 1;
        let mut last_output = 0;
        let mut last_e = 0;

        while code != 99 {
            for x in 0..5 {
                code = execute(&mut programs[x], &mut indices[x], amps[x], &mut last_output);
                if x == 4 {
                    last_e = last_output.clone();
                }
            }
        }

        if last_e > max_out {
            max_out = last_e;
        }
    } 

    println!("MAX: {}", max_out);
}