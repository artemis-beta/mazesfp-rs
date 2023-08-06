use crate::direction::{Direction, route, next_cell};
use crate::grid::Grid;


pub fn algorithm(grid: &mut Grid) -> () {
    let mut visited = Vec::<Vec::<usize>>::new();
    let mut current_cell = vec![0, 0];

    while visited.len() < grid.height() * grid.width() {
        let direction = route(grid, &current_cell);
        let new_position = next_cell(&direction, &current_cell);

        let been_here: bool = visited.iter().any(|a| a == &new_position);

        if !been_here {
            match direction {
                Direction::East => grid.erase_wall(current_cell[0], current_cell[1], false),
                Direction::West => grid.erase_wall(current_cell[0]-1, current_cell[1], false),
                Direction::North => grid.erase_wall(current_cell[0], current_cell[1]-1, true),
                _ => grid.erase_wall(current_cell[0], current_cell[1], true)
            };
        }

        current_cell = new_position.clone();

        if !been_here {
            visited.push(current_cell.clone());
        }

    }
}