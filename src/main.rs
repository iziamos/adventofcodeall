use std::{collections::HashSet, fmt, fs::read_to_string};


fn main() {
    let input = read_to_string("./inputs/8.txt").unwrap();
    let  grid: Vec<Vec<char>> =
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

    // for a in antinodes {
    //     grid[a.i][a.j] = '#';
    // }
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         print!("{}", grid[i][j]);
    //     }
    //     println!();
    // }
    //println!();

    println!("Counted {} unique antinodes", antinodes.len());
}

fn get_antinodes(grid: &Vec<Vec<char>>, antennas: &Vec<Position>) -> Vec<Position> {
    let mut ret = vec![];

    for b in 0..antennas.len() {
        for h in 1..antennas.len() {
            let first = antennas[b];
            let second = antennas[h];

            let idistance = second.i.abs_diff(first.i);
            let jdistance = second.j.abs_diff(first.j);

            if first.i > second.i &&
               first.j > second.j {

                if second.i >= idistance && second.j >= jdistance{
                    let a = Position {i: second.i - idistance, j: second.j - jdistance};
                    ret.push(a);
                }

                let b = Position {i: first.i + idistance , j: first.j + jdistance};
                if is_in_grid(&grid, &b) {
                    ret.push(b);
                }
            }
            if first.i > second.i &&
               first.j < second.j {

                if first.j >= jdistance {
                    let a = Position {i: first.i + idistance, j: first.j - jdistance};
                    if is_in_grid(&grid, &a) {
                        ret.push(a);
                    }
                }

                if second.i >= idistance {
                    let b = Position {i: second.i - idistance, j: second.j + jdistance};
                    if is_in_grid(&grid, &b) {
                        ret.push(b);
                    }
                }
            }
            if first.i < second.i &&
               first.j > second.j {

                if first.i >= idistance {
                    let a = Position {i: first.i - idistance, j: first.j + jdistance};
                    if is_in_grid(&grid, &a) {
                        ret.push(a);
                    }
                }

                if second.j >= jdistance {
                    let b = Position {i: second.i + idistance, j: second.j - jdistance};
                    if is_in_grid(&grid, &b) {
                        ret.push(b);
                    }
                }
            }
            if first.i < second.i &&
               first.j < second.j  {

                if first.i >= idistance && first.j >= jdistance {
                    let a = Position {i: first.i - idistance, j: first.j - jdistance};
                    ret.push(a);
                }

                let b = Position {i: second.i + idistance, j: second.j + jdistance};
                if is_in_grid(&grid, &b) {
                    ret.push(b);
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
    p.i < grid.len() && p.j < grid[0].len()
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
                ret.push(Position { i, j });
            }
        }
    }
    visited.push(a);
    return ret;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    i: usize,
    j: usize
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position({}, {})", self.i, self.j)
    }
}
