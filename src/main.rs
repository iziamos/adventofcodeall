use std::{collections::HashSet, fmt, fs::read_to_string};


fn main() {
    let input = read_to_string("./inputs/s.txt").unwrap();
    let mut grid: Vec<Vec<char>> =
       input.lines()
            .map(|s| s.chars().collect())
            .collect();

    let mut antinodes: HashSet<Position> = HashSet::new();
    let mut visited = vec![];

    loop {
        let a = find_antennas(&grid, &mut visited);
        if a.is_empty() {
            break;
        }

        let at = get_antinodes(&grid, &a);
        antinodes.extend(at);
    }

    for a in antinodes {
        grid[a.i as usize][a.j as usize] = '#';
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
    println!();

    // println!("Counted {} unique antinodes", antinodes.len());
}

fn get_antinodes(grid: &Vec<Vec<char>>, antennas: &Vec<Position>) -> Vec<Position> {
    let mut ret = vec![];

    for b in 0..antennas.len() {
        for h in 1..antennas.len() {
            let mut first = antennas[b];
            let mut second = antennas[h];

            let idistance = second.i.abs_diff(first.i);
            let jdistance = second.j.abs_diff(first.j);

            if first.i > second.i &&
               first.j > second.j { // A
                loop {
                    let p = Position{i: first.i + idistance as i32, j: first.j + jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        first = p;
                    }
                    else {
                        break;
                    }
                }
                loop {
                    let p = Position{i: second.i - idistance as i32, j: second.j - jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        second = p;
                    }
                    else {
                        break;
                    }
                }
            }
            if first.i > second.i &&
               first.j < second.j { // B
                loop {
                    let p = Position{i: first.i + idistance as i32, j: first.j - jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        first = p;
                    }
                    else {
                        break;
                    }
                }
                loop {
                    let p = Position{i: second.i - idistance as i32, j: second.j + jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        second = p;
                    }
                    else {
                        break;
                    }
                }
            }
            if first.i < second.i &&
               first.j > second.j { // C
                loop {
                    let p = Position{i: first.i - idistance as i32, j: first.j + jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        first = p;
                    }
                    else {
                        break;
                    }
                }
                loop {
                    let p = Position{i: first.i + idistance as i32, j: first.j - jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        second = p;
                    }
                    else {
                        break;
                    }
                }
            }
            if first.i < second.i &&
               first.j < second.j { // D
                loop {
                    let p = Position{i: first.i - idistance as i32, j: first.j - jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        first = p;
                    }
                    else {
                        break;
                    }
                }
                loop {
                    let p = Position{i: second.i + idistance as i32, j: second.j + jdistance as i32};
                    if is_in_grid(grid, &p) {
                        ret.push(p);
                        second = p;
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }
    return ret;
}

fn is_in_grid(grid: &Vec<Vec<char>>, p: &Position) -> bool {
    if grid.len() == 0 {
        return false;
    }
    p.i < grid.len()  as i32 && p.j < grid[0].len() as i32
}

fn find_antennas(grid: &Vec<Vec<char>>, visited: &mut Vec<char>) -> Vec<Position> {
    let mut a = '.';
    let mut ret: Vec<Position> = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];

            if c == '.' || visited.contains(&c) {
                continue;
            }

            if a == '.' {
                a = c;
            }

            if c == a {
                ret.push(Position { i: i as i32, j: j as i32 });
            }
        }
    }
    visited.push(a);
    return ret;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    i: i32,
    j: i32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position({}, {})", self.i, self.j)
    }
}
