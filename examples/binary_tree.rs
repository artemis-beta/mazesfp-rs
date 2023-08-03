use pp_mazes::binary_tree;
use pp_mazes::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    binary_tree::algorithm(&mut grid);
    print!("{}", grid);
}
