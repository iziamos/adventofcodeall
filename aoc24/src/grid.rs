use std::fs::read_to_string;
use std::fmt;

pub struct Grid{
    data: Vec<Vec<char>>
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

    fn char_at_unsafe(&self, x: isize, y: isize) -> char {
        self.data[y as usize][x as usize]
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