use std::{fs::read_to_string};

fn main() {
    let input = read_to_string("./inputs/4.txt").unwrap();
    let grid = input.lines()
        .map(|l| l.chars())
        .map(|l| l.collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();



    let ret = search_horizontal(&input) +
        search_vertical(&grid) +
        search_left_right_diag(&grid) +
        search_right_left_diag(&grid);

    println!("Result is {ret}");

    println!("{} {} {} {}", search_horizontal(&input) ,
    search_vertical(&grid) ,
    search_left_right_diag(&grid) ,
    search_right_left_diag(&grid));

}

fn search_horizontal(input: &str) -> u128 {
    let mut ret = 0;

    let lines = input.lines().collect::<Vec<&str>>();
    let lcount = lines.len();
    let llen = lines[0].len();


    for i in 0..lcount {
        let l = lines[i].chars().collect::<Vec<char>>();
        for j in 0..llen - 3 {
            if l[j] == 'X' && l[j + 1] == 'M'&& l[j + 2] == 'A'&& l[j + 3] == 'S' {
                ret += 1;
            }
            if l[j] == 'S' && l[j + 1] == 'A'&& l[j + 2] == 'M'&& l[j + 3] == 'X' {
                ret += 1;
            }
        }
    }

    return ret;
}

fn search_vertical(grid: &Vec<Vec<char>>) -> u128 {
    let mut ret = 0;

    for x in 0..grid[0].len() {
        print!("{}", grid[0][x]);
    }
    println!("");

    for i in 0..grid[0].len() - 3 {
        for j in 0..grid.len() {

            if grid[i][j] == 'X' &&
                grid[i + 1][j ] == 'M' &&
                grid[i + 2][j ] == 'A' &&
                grid[i + 3][j ] == 'S' {
                ret += 1;
                continue;
            }

            if grid[i][j] == 'S' &&
                grid[i + 1][j] == 'A' &&
                grid[i + 2][j] == 'M' &&
                grid[i + 3][j] == 'X'{
                ret += 1;
            }
        }
    }
    return ret;
}

fn search_left_right_diag(grid: &Vec<Vec<char>>) -> u128 {
    let mut ret = 0;

    for i in 0..grid[0].len() - 3{
        for j in 0..grid.len() - 3 {

            if grid[i][j] == 'X' &&
                grid[i + 1][j + 1] == 'M' &&
                grid[i + 2][j + 2] == 'A' &&
                grid[i + 3][j + 3] == 'S' {

                ret += 1;
                continue;
            }

            if grid[i][j] == 'S' &&
                grid[i + 1][j + 1] == 'A' &&
                grid[i + 2][j + 2] == 'M' &&
                grid[i + 3][j + 3] == 'X'{

                ret += 1;
            }
        }
    }
    return ret;
}

fn search_right_left_diag(grid: &Vec<Vec<char>>) -> u128 {
    let mut ret = 0;

    for i in 0..grid[0].len() - 3{
        for j in 3..grid.len() {

            if grid[i][j] == 'X' &&
                grid[i + 1][j - 1] == 'M' &&
                grid[i + 2][j - 2] == 'A' &&
                grid[i + 3][j - 3] == 'S' {

                ret += 1;
                continue;
            }

            if grid[i][j] == 'S' &&
                grid[i + 1][j - 1] == 'A' &&
                grid[i + 2][j - 2] == 'M' &&
                grid[i + 3][j - 3] == 'X'{
                ret += 1;
            }
        }
    }
    return ret;
}
