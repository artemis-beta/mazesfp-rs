use rand::{thread_rng, Rng};
use crate::grid::Grid;

pub fn algorithm(grid: &mut Grid) -> (){
    let mut rng = thread_rng();

    for j in 0..grid.height() {
        for i in 0..grid.width() { 
            let erase_south = rng.gen_bool(0.5);
            if i == grid.width() - 1 {
                grid.erase_wall(j, i, true);
            } else if i == grid.height() - 1 {
                grid.erase_wall(j, i, false);
            } else {
                grid.erase_wall(j, i, erase_south);
            }
        }
    }
}