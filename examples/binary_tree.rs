use mazesfp::binary_tree;
use mazesfp::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    binary_tree::algorithm(&mut grid);
    print!("{}", grid);
}
