use std::fs::read_to_string;

mod grid;

fn main() {
    let input = read_to_string("./inputs/9.txt").unwrap();
    let mut f: Vec<i128> = vec![];
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        if i % 2 == 0 {
            let n = chars[i] as i32 - '0' as i32;
            for x in 0..n {
                f.push(i as i128 / 2);
            }
        }
        else {
            let n = chars[i] as i32 - '0' as i32;
            for x in 0..n {
                f.push(-1);
            }
        }
    }

    let mut e = (f.len() - 1) as isize;
    let mut s = 0;

    loop {
        if e <=s {

            break;
        }
        if f[e as usize] == -1 {
            e -= 1;
            continue;
        }
        while f[s as usize] != -1 {
            s += 1;
        }
        if e <=s {
            
            break;
        }
        f[s as usize] = f[e as usize];
        f[e as usize] = -1;
        // print_vec(&f);
    }


    let mut result: i128 = 0;

    for i in 0..f.len() {
        let c = f[i];

        if c == -1 {
            break;
        }
        result += (c * i as i128) as i128;

    }
    println!("Result is: {result}")



}

fn print_vec(data: &Vec<i128>) {
    for i in data {
        if *i == -1 {
            print!(".");
        }
        else {
            print!("{i}");
        }
    }
    println!();
}


