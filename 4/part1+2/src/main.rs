fn check_num(num: i32) -> bool {

    let mut double_exists = false;
    let num_split: Vec<u32> = num.to_string().chars().map(|c| c.to_digit(10).expect("Can't convert char to digit!")).collect();
    for i in 1..num_split.len() {
        if num_split[i-1] > num_split[i] {
            return false;
        } else if num_split[i-1] == num_split[i] {
            let mut holder = true;;
            if i != 1 && num_split[i-2] == num_split[i] {
                holder = false;
            }
            if i != num_split.len()-1 && num_split[i+1] == num_split[i] {
                holder = false;
            }
            if holder {
                double_exists = true;
            }
        }
    }
    double_exists
}

fn main() {

    let mut nums_matching = 0;

    for i in 245182..790572 {
        if  check_num(i) {
            nums_matching += 1;
        }
    }

    println!("Possible passwords: {}", nums_matching);
}
