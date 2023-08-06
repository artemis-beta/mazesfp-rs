use mazesfp::hunt_and_kill;
use mazesfp::grid;

fn main() {
    let mut grid = grid::build_grid(10, 10);
    hunt_and_kill::algorithm(&mut grid);
    print!("{}", grid);
}
