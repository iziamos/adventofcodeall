use std::{fs::read_to_string};

fn main() {
    let mut result = 0;
    for line in read_to_string("./inputs/sample.txt").unwrap().lines() {
        let parts: Vec<i32> = line.split(" ")
        .map(|e| e.parse::<i32>().expect("Parse fail"))
        .collect();

        let mut r = is_safe(&parts);
        
        for i in 0..parts.len() {
            let mut c = parts.clone();
            c.remove(i);
            r += is_safe(&c);
        }


        if r > 0 {
            result += 1;
        }
    }
    println!("Number of safe rows: {}", result);
}

fn is_safe(v: &Vec<i32>) ->  i32 {

    let fm = mode(v[0], v[1]);

    for i in 0..v.len() - 1 {
        let left: i32 = v[i];
        let right: i32 = v[i + 1];
        if fm != mode(left, right) {            
            return 0;
        }

        let distance = abs(left - right);
        if distance < 1 || distance > 3 {
            return 0;
        }
    }
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
