use std::{fs::read_to_string};

fn main() {
    let mut result = 0;
    for line in read_to_string("./inputs/2.txt").unwrap().lines() {
        let parts: Vec<i32> = line.split(" ")
        .map(|e| e.parse::<i32>().expect("Parse fail"))
        .collect();

        result += is_safe(parts);
    }
    println!("Number of safe rows: {}", result);
}

fn is_safe(v: Vec<i32>) ->  i32 {

    let fm = mode(v[0], v[1]);

    for i in 0..v.len() - 1 {
        let left: i32 = v[i];
        let right: i32 = v[i + 1];
        if fm != mode(left, right) {
            println!("Unsafe: {}, {}, {}, {}, {}", v[0],v[1],v[2],v[3],v[4]);
            return 0;
        }

        let distance = abs(left - right);
        if distance < 1 || distance > 3 {
            println!("Unsafe: {}, {}, {}, {}, {}", v[0],v[1],v[2],v[3],v[4]);
            return 0;
        }
    }
    println!("Safe: {}, {}, {}, {}, {}", v[0],v[1],v[2],v[3],v[4]);
     return 1;
}

fn mode(left: i32, right: i32) -> i32 {
    if left > right {
        return 1;
    }
    else {
        return -1;
    }
}

fn abs(i: i32) -> i32 {
    if i < 0
    { return -i} 
    else { return i;}
}
