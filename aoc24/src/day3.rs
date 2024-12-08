use std::{fs::read_to_string};
use regex::Regex;


fn main() {
    let input = read_to_string("./inputs/3.txt").unwrap();
    let expression = r"(mul[(][0-9]{1,3}[,][0-9]{1,3}[)]|don[']t[(][)])|(do[(][)])";
    let regex = Regex::new(expression).expect("Regex issue");


    let mut enabled = true;
    let mut result = 0;
    for c in regex.find_iter(&input) {
        //println!("{}", c.as_str());

        let s = c.as_str();
        if s.eq("don't()") {
            enabled = false;
            continue;
        }
        if s.eq("do()") {
            enabled = true;
            continue;
        }
        if enabled {
            result += compute(s);
        }
    }
    println!("Result is: '{result}'")
}

fn compute(val: &str) -> i128 {

    let vals = &val[4..val.len() - 1];
    let parts: Vec<_> = vals.split(",").collect();
    let first: i128 = parts.first().expect("Couldnt find first").parse().expect("couldnt parse");
    let second: i128 = parts.last().expect("Couldnt find last").parse().expect("couldnt parse");

    return first * second;
}

