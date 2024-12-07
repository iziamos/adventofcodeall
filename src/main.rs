use std::fs::read_to_string;


fn main() {
    let input = read_to_string("./inputs/7.txt").unwrap();

    let mut result = 0;
    for line  in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();

        let head: u128 = split.first().expect("asda").parse::<u128>().expect("Head parse error.");
        let tail = split.last().expect("blah");
        let numbers: Vec<u128> = tail.split(" ").map(|n| n.parse::<u128>().expect("oasd qw")).collect();

        let valid = is_valid(head, &numbers);

        if valid {
            result += head;
        }
    }
    println!("Result: {result}");
}

fn is_valid(v: u128, values: &Vec<u128>) -> bool{
        let mask =  (1 << values.len()) - 1;

        for i in 0..mask {
            let mut r: u128 = values[0];
            for j in 1..values.len() {
                // println!("{i} {j}");
                let addition = (1 << j - 1) & i == 0;
                if addition {
                    r += values[j];
                }
                else {
                    r *= values[j];
                }
            }

            if v == r {
                return true;
            }
        }
        return false;
}
