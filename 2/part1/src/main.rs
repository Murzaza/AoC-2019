use std::fs;
use std::convert::TryInto;

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Unable to read file");
    let mut inputs: Vec<u32> = input.as_str().split(',')
                                   .map(|x| x.parse::<u32>().expect("Couldn't convert string to i64"))
                                   .collect();

    //Restore 1202 error code.
    inputs[1] = 12;
    inputs[2] = 2;

    //Go through the program an execute.
    for i in (0..inputs.len()).step_by(4) {
        println!("Checking value at {}:{}", i, inputs[i]);

        let dest: usize = inputs[i+3].try_into().unwrap();
        let a: usize = inputs[i+1].try_into().unwrap();
        let b: usize = inputs[i+2].try_into().unwrap();

        match inputs[i] {
            1 => {
                    inputs[dest] = inputs[a] + inputs[b];
                 },
            2 => {
                    inputs[dest] = inputs[a] * inputs[b];
                 },
            99 => {
                    println!("Exiting at {}, Inputs:\n {:?}", i, inputs);
                    return;
                  },
            _ => {/* No Op */}
        }
    }

    println!("Exiting at end, Pos 0: {}", inputs[0]);
}

