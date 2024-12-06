use std::{fs::read_to_string};

fn main() {
    let input = read_to_string("./inputs/6.txt").unwrap();
    let mut grid: Vec<Vec<char>> =
       input.lines()
            .map(|s| s.chars().collect())
            .collect();

    let mut result = 0;

    let mut p = find_start_position(&grid);
    loop {
        let n = next_position(&grid, &p);
        if grid[n.y][n.x] == '.' {
            result += 1;
            grid[n.y][n.x] = '@';
        }

        if  grid[n.y][n.x] == '#' {
            p = rotate(&p);
        }
        else {
            p = n;
        }
        println!("{result}");
    }
}

fn next_position (grid: &Vec<Vec<char>>, current: &Position) -> Position {
    return match current.direction {
        Direction::Up => Position{x: current.x, y : current.y - 1, direction: current.direction},
        Direction::Left => Position{x: current.x - 1, y: current.y, direction: current.direction},
        Direction::Right => Position{x: current.x + 1, y: current.y, direction: current.direction},
        Direction::Down => Position{x: current.x, y: current.y + 1, direction: current.direction},
    }
}

fn rotate (current: &Position) -> Position {
    return match current.direction {
        Direction::Up => Position{x: current.x, y : current.y, direction: Direction::Right},
        Direction::Left => Position{x: current.x, y: current.y, direction: Direction::Up},
        Direction::Right => Position{x: current.x, y: current.y, direction: Direction::Down},
        Direction::Down => Position{x: current.x, y: current.y, direction: Direction::Left},
    }
}


fn find_start_position(grid: &Vec<Vec<char>>) -> Position{
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return Position{x, y, direction: Direction::Up}
            }
            if grid[y][x] == 'v' {
                return Position {x, y, direction: Direction::Down}
            }
            if grid[y][x] == '>' {
                return Position {x, y, direction: Direction::Right}
            }
            if grid[y][x] == '<' {
                return Position {x, y, direction: Direction::Left}
            }
        }
    }
    panic!("Couldnt find starting position.");
}

struct Position {
    x: usize,
    y: usize,
    direction: Direction
}

enum State {
    Blocked,
    Visited,
    Unvisited
}


#[derive(Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}