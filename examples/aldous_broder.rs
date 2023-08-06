use mazesfp::aldous_broder;
use mazesfp::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    aldous_broder::algorithm(&mut grid);
    print!("{}", grid);
}
