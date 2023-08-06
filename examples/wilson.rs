use mazesfp::wilson;
use mazesfp::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    wilson::algorithm(&mut grid);
    print!("{}", grid);
}
