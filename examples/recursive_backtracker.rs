use mazesfp::recursive_backtracker;
use mazesfp::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    recursive_backtracker::algorithm(&mut grid);
    print!("{}", grid);
}
