use std::fs::read_to_string;
use std::fmt;

pub struct Grid{
    data: Vec<Vec<char>>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    row: usize,
    col: usize,
    val: char,
}

impl Grid {
    pub fn from_file(file: &str) -> Grid {
        let input = read_to_string(file).expect("Failed to open file");
        let grid: Vec<Vec<char>> =
        input.lines()
             .map(|s| s.chars().collect())
             .collect();

        Grid { data: grid }
    }

    fn is_in_bounds(&self, row: isize, col:  isize) -> bool {
        col > -1 &&
        row > -1 &&
        row < self.data.len() as isize &&
        col < self.data[0].len() as isize
    }

    fn char_at_unsafe(&self, row: isize, col: isize) -> char {
        self.data[row as usize][col as usize]
    }

    fn char_at(&self, row: isize, col: isize) -> Option<char> {
        if self.is_in_bounds(row, col) {
            return Some(self.data[row as usize][col as usize])
        }
        None
    }

    fn width(&self) -> usize {
        return self.data.len();
    }

    fn height(&self) -> usize {
        if self.data.len() == 0 {
            return 0;
        }
        return self.data[0].len();
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            row: 0,
            col: -1,
        }
    }

}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    row: isize,
    col: isize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {

        self.col += 1;

        if self.col as usize == self.grid.width() {
            self.col = 0;
            self.row += 1;
        }

        if self.row as usize == self.grid.height() {
            return None;
        }

        let p = Point {
            row: self.row as usize,
            col: self.col as usize,
            val: self.grid.char_at_unsafe(self.row, self.col) };

        Some(p)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            // Join each row of chars into a string and print it
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point row: {}, col: {}, val: {}", self.row, self.col, self.val )?;
        Ok(())
    }
}
