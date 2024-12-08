use std::{fs::read_to_string, thread};


fn main() {
    let input = read_to_string("./inputs/7.txt").unwrap();

    let mut result = 0;

    let clone = input.clone();
    let clone2 = input.clone();

    let handle1 = thread::spawn(move || {
        let lines: Vec<&str> = clone.lines().collect();
        let split_lines = lines.split_at(lines.len() / 2);

        let mut r = 0;
        for line  in split_lines.0 {
            r += process_line(line);
        }
        r
    });

    let handle2 = thread::spawn(move || {
        let lines: Vec<&str> = clone2.lines().collect();
        let split_lines = lines.split_at(lines.len() / 2);

        let mut r = 0;
        for line  in split_lines.1 {
            r += process_line(line);
        }
        r
    });

    result += handle1.join().unwrap();
    result += handle2.join().unwrap();

    println!("Result: {result}");
}

fn process_line(line: &str) -> u128 {
    let split: Vec<&str> = line.split(": ").collect();

    let head: u128 = split.first().expect("asda").parse::<u128>().expect("Head parse error.");
    let tail = split.last().expect("blah");
    let numbers: Vec<u128> = tail.split(" ").map(|n| n.parse::<u128>().expect("oasd qw")).collect();


    let mask =  (1 << numbers.len()) - 1;
    let mask2 =  (1 << numbers.len()) - 1;

    for m in 0..mask2 {
        for i in 0..mask {
            let mut r: u128 = numbers[0];
            for j in 1..numbers.len() {
                let addition = (1 << j - 1) & i == 0;
                let concat = (1 << j - 1) & m == 0; // we do the concat worktwice
                if concat {
                    let d = count_digits(numbers[j]);
                    for _a in 0..d {
                        r *= 10;
                    }
                    r += numbers[j];
                }
                else if addition {
                    r += numbers[j];
                }
                else {
                    r *= numbers[j];
                }

                if r > head {
                    break;
                }
            }

            if r == head {
                return head;
            }
        }
    }
    return 0;
}


fn count_digits(num: u128) -> u32 {
    if num == 0 {
        return 1;
    }
    let mut count = 0;
    let mut n = num;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}
