use std::{collections::HashSet, fmt, fs::read_to_string};

use grid::{Grid, Point};

mod grid;

fn main() {
    let mut g = grid::Grid::from_file(&"./inputs/s.txt");

    for i in 0..g.width() {
        g[i][0] = '@';
        for j in 0..g.height() {
            print!("{}", g[i][j]);
        }
        println!();
    }
    println!();


    println!("{}", g);
}



