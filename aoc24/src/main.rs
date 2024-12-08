use grid::Grid;

mod grid;

fn main() {
    let mut g = Grid::from_file(&"./inputs/s.txt");

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



