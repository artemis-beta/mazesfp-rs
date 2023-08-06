use rand::{thread_rng, Rng};
use crate::grid::Grid;

pub fn algorithm(grid: &mut Grid) -> (){
    let mut rng = thread_rng();

    for j in 0..grid.height() {
        for i in 0..grid.width() { 
            let erase_south = rng.gen_bool(0.5);

            let at_limit_x = i == grid.width() - 1;
            let at_limit_y = j == grid.height() - 1;

            if at_limit_x && !at_limit_y {
                grid.erase_wall(i, j, true);
            } else if at_limit_y && !at_limit_x {
                grid.erase_wall(i, j, false);
            } else if !at_limit_x && !at_limit_y {
                grid.erase_wall(i, j, erase_south);
            }
        }
    }
}