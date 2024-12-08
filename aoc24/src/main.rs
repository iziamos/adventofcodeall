
mod grid;

fn main() {
    let d = grid::Grid::from_file(&"./inputs/s.txt");

    println!("{d}");
    for p in d.iter() {
        println!("{}", p);
    }
}