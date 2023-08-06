use rand::{thread_rng, Rng};
use crate::grid::Grid;

pub fn algorithm(grid: &mut Grid) {
    let mut rng = thread_rng();

    let mut coord_set: Vec<Vec<usize>> = Vec::<Vec::<usize>>::new();

    for j in 0..grid.height() {
        for i in 0..grid.width() {
            coord_set.push(vec![i, j]);
            let index = rng.gen_range(0..coord_set.len());
            let erase = rng.gen_bool(0.5);

            let x = coord_set[index][0];
            let y = coord_set[index][1];
    
            if erase && y > 0 {
                grid.erase_wall(x, y-1, true);
                coord_set.clear();
            }
            else if i < grid.width() - 1 {
                grid.erase_wall(i, j, false);
            }
        }
    }
}