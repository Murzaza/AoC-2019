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

    for i in 0..(WIDTH * HEIGHT) {
        if i % WIDTH == 0 {
            println!("");
        }
        for l in 0..layers.len() {
            if layers[l][i] < 2 {
                let output = match layers[l][i] {
                    0 => " ",
                    1 => "@",
                    _ => "X"
                };
                print!("{}",output);
                break;
            }
        } 
    }
}
