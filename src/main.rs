use std::{fs::read_to_string};

fn main() {
    let mut left: [i64; 1024] =  [0; 1024];
    let mut right: [i64; 1024] =  [0; 1024];


    let mut i = 0;
    for line in read_to_string("./inputs/1.txt").unwrap().lines() {
        let parts: Vec<_> = line.split("   ").collect();

        let f : i64 = parts.first().expect("Didnt find first").parse().expect("Couldnt parse");
        let l : i64 = parts.last().expect("Didnt find first").parse().expect("Couldnt parse");
        left[i] = f;
        right[i] = l;
        i += 1;
    }

    let mut sum: i64 = 0;
    for i in 0..1024 {
        sum += left[i] * count(right, left[i]);
    }

    println!("Answer is: '{}'", sum);
}

fn count(array: [i64; 1024], value: i64) ->  i64 {
    let mut ret = 0;
    for i in 0..1024 {
        if array[i] == value {
            ret += 1;
        }
    }
    return ret;
}