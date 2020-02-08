use std::fs;
use std::convert::TryInto;
use itertools::Itertools;

fn get_program(filename: &str) -> Vec<i32> {
    let input = fs::read_to_string(filename).expect("Unable to read file");
    input.as_str().split(',')
        .map( |x| x.parse::<i32>().expect("Couldn't convert string to i32") )
        .collect()
}

fn get_inputs(code: i32, a: i32, b: i32, inputs: &Vec<i32>) -> (i32, i32) {
    match code {
        0 => (inputs[a as usize], inputs[b as usize]),
        1 => (a, inputs[b as usize]),
        10 => (inputs[a as usize], b),
        11 => (a, b),
        _ => (0,0) //Bruh
    }
}

fn execute(inputs: &mut Vec<i32>, phase: i32, input: &mut i32) {
    let mut i: usize = 0;
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
                    //let val = io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().expect("Can't convert user input to i32");
                    if first {
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
                    i += 2
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
                    return;
                  },
            _ => {
                    println!("We shouldn't be here inputs[{}] = {} -> {}", i, inputs[i], inputs[i] % 100);
                    return
                 }
        };
    }
}


fn main() {
    let program = get_program("resources/input.txt");
    let mut output: i32;
    let mut max_out = 0;

    for amps in (0..5).permutations(5) {
        output = 0;
        for amp in amps.clone() {
            let mut prog = program.clone();
            execute(&mut prog, amp,&mut output);
        }

        if output > max_out {
            println!("For {:?} => {}", amps, output);
            max_out = output;
        }
    } 

    println!("MAX: {}", max_out);
}