use std::fs;

static WIDTH: usize = 25;
static HEIGHT: usize = 6;

fn get_input() -> Vec<Vec<i32>> {
    let image = fs::read_to_string("resources/input.txt").expect("Unable to read file");

    let mut data: Vec<i32> = image.chars().map( |c| c.to_string().parse::<i32>().expect("Unable to convert string to i32") ).collect();

    let mut layers = Vec::new();
    while data.len() > (WIDTH * HEIGHT) {
        let next = data.split_off(WIDTH * HEIGHT);
        layers.push(data);
        data = next;
    }
    layers.push(data);

    layers
}

fn main() {
    let layers = get_input();
    let mut low_product = 0; 
    let mut low_zeros = 100000;
    for layer in layers {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for pixel in layer {
            match pixel {
                0 => zeros += 1,
                1 => ones += 1,
                2 => twos += 1,
                _ => println!("We shouldn't be here")
            };
        }

        if zeros < low_zeros {
            low_product = ones * twos;
            low_zeros = zeros;
        }
    }

    println!("Lowest amount of zeros: {}, with a product of {}", low_zeros, low_product);
}
