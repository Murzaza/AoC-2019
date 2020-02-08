use std::fs;
use std::convert::TryInto;

fn find_noun_verb(noun: u32, verb: u32) -> u32 {
    let input = fs::read_to_string("resources/input.txt").expect("Unable to read file");
    let mut inputs: Vec<u32> = input.as_str().split(',')
                                   .map(|x| x.parse::<u32>().expect("Couldn't convert string to i64"))
                                   .collect();

    //Restore 1202 error code.
    inputs[1] = noun;
    inputs[2] = verb;

    //Go through the program an execute.
    for i in (0..inputs.len()).step_by(4) {

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
                    return inputs[0];
                  },
            _ => {/* No Op */}
        }
    }

    return inputs[0];
}

fn main() {
    for n in 0..99 {
        for v in 0..99 {
            let x = find_noun_verb(n,v);
            if x == 19690720 {
                println!("Found It {}", 100*n+v);
                return;
            }
        }
    }
}