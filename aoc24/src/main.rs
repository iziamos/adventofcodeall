
mod grid;

fn main() {
    let d = grid::Grid::from_file(&"./inputs/s.txt");

    println!("{d}");
}