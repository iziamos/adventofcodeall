use std::{fs::read_to_string};

fn main() {
    let input = read_to_string("./inputs/4.txt").unwrap();
    let grid = input.lines()
        .map(|l| l.chars())
        .map(|l| l.collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();



    let ret = search_vertical(&grid) ;
    println!("Result is {ret}");
}


fn search_vertical(grid: &Vec<Vec<char>>) -> u128 {
    let mut ret = 0;

    for i in 0..grid[0].len() - 2 {
        for j in 0..grid.len()- 2 {

            if grid[i][j] == 'M' &&
                grid[i + 1][j + 1 ] == 'A' &&
                grid[i + 2][j + 2] == 'S' &&
                grid[i + 2][j] == 'M' &&
                grid[i][j + 2] == 'S'
                {
                    ret += 1;
                    continue;
                }

                if grid[i][j] == 'S' &&
                    grid[i + 1][j + 1 ] == 'A' &&
                    grid[i + 2][j + 2] == 'M' &&
                    grid[i + 2][j] == 'S' &&
                    grid[i][j + 2] == 'M'
                {
                    ret += 1;
                    continue;
                }

                if grid[i][j] == 'S' &&
                    grid[i + 1][j + 1 ] == 'A' &&
                    grid[i + 2][j + 2] == 'M' &&
                    grid[i + 2][j] == 'M' &&
                    grid[i][j + 2] == 'S'
                    {
                        ret += 1;
                        continue;
                    }

                    if grid[i][j] == 'M' &&
                    grid[i + 1][j + 1 ] == 'A' &&
                    grid[i + 2][j + 2] == 'S' &&
                    grid[i + 2][j] == 'S' &&
                    grid[i][j + 2] == 'M'
                    {
                        ret += 1;
                        continue;
                    }
        }
    }
    return ret;
}
