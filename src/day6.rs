use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input = read_to_string("./inputs/6.txt").unwrap();
    let mut grid: Vec<Vec<char>> =
       input.lines()
            .map(|s| s.chars().collect())
            .collect();

    let mut placements: HashSet<Position> = HashSet::new();

    let z = find_start_position(&grid);
    let mut p = z.clone();

    while can_proceed(&grid, &p){
        let n = next_position( &p);

        if grid[n.y][n.x] == '#' {
            p = rotate(&p);
        }
        else {
            if n.x == z.x && n.y == z.y {
                p = n;
                continue;
            }

            if grid[n.y][n.x] == '+' {
                p = n;
                continue;
            }

            grid[n.y][n.x] = '#';
            if see_if_i_end_up_in_loop(&mut grid, &p) {
                placements.insert(n);
            }

            grid[n.y][n.x] = '+';
            p = n;
        }
    }

    println!("Result is: '{}'", placements.len());

}

fn see_if_i_end_up_in_loop (grid: &mut Vec<Vec<char>>, start: &Position) -> bool {
    let mut visited: Vec<Position> = vec![];
    let mut p = start.clone();

    while can_proceed(&grid, &p) {
        let n = next_position(&p);

        if grid[n.y][n.x] != '#' {
            p = n;
        }
        else {
            if visited.contains(&p) {
                return true;
            }
            visited.push(p);
            p = rotate(&p);
        }
    }
    return false;
}

fn can_proceed (grid: &Vec<Vec<char>>, current: &Position) -> bool {
    match current.direction {
        Direction::Up => return current.y != 0,
        Direction::Left => return current.x != 0,
        Direction::Right => return current.x != grid[0].len() - 1,
        Direction::Down => return current.y != grid.len() - 1
    }
}

fn next_position (current: &Position) -> Position {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}