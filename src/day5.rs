use std::{fs::read_to_string, cmp::Ordering};

fn main() {
    let input = read_to_string("./inputs/5.txt").unwrap();
    let str_lines: Vec<&str> = input.lines().collect();


    let mut orders: Vec<Order> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];



    for ln in str_lines {
        if ln.contains("|") {
            let s = ln.split("|");
            let o = from_vector(s);
             
            orders.push(o)
        }

        if ln.contains(",") {
            let p: Vec<i32> = ln.split(",")
                .map(|s| s.parse::<i32>().expect("Blew up parsing string"))
                .collect();

            updates.push(p);
        }
    }

    let mut result = 0;
    'outer: for u in updates {
        for i in 1..u.len() {
            let update: i32 = u[i];
            let previous: &[i32] =  &u[0..i-1];

            if has_illegals(orders.clone(), previous, update) {
                let mut c = u.clone();
                c.sort_by(|x, y| compare(&orders, x, y));
                let b = c[c.len() / 2];
                println!("{b}");
                result += b;
                continue 'outer;
            }
        }
    }



    println!("Result: {}", result);
}

#[derive(Clone)]
struct Order {
    left: i32,
    right: i32
}

fn compare(ord: &Vec<Order>, a: &i32, b: &i32) -> Ordering {

    for o in ord {
        if o.left == (a + 0) && o.right == (b + 0) {
            return Ordering::Less;
        }
    }

    for o in ord {
        if o.left == (b + 0) && o.right == (a + 0) {
            return Ordering::Greater;
        }
    }

    return Ordering::Equal;
}

fn from_vector(vs: std::str::Split<'_, &str>) -> Order {
    let v: Vec<&str> = vs.collect();
    Order{
        left: v[0].parse::<i32>().expect("Failed to parse"),
        right: v[1].parse::<i32>().expect("Failed to parse")}
}

fn has_illegals(orderings: Vec<Order>, previous: &[i32], v: i32) -> bool {
    for p in previous {
        for o in orderings.clone() {
            if o.left == v && o.right == (p + 0){
                return true;
            }
        }
    }
    return false;
}