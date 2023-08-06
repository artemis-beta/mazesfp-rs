use mazesfp::sidewinder;
use mazesfp::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    sidewinder::algorithm(&mut grid);
    print!("{}", grid);
}
