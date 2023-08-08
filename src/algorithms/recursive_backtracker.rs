use crate::direction::{Direction, route, next_cell};
use crate::grid::Grid;

pub fn visited_with_unvisited(visited: &Vec<Vec<usize>>, position: Vec<usize>, width: usize, height: usize) -> Option<Direction> {
    if !visited.iter().any(|a| a == &vec![position[0].saturating_add(1), position[1]]) {
        if position[0] < width - 1 {return Some(Direction::East);}
    }

    if !visited.iter().any(|a| a == &vec![position[0].saturating_sub(1), position[1]]) {
        if position[0] > 0 {return Some(Direction::West);}
    }

    if !visited.iter().any(|a| a == &vec![position[0], position[1].saturating_add(1)]) {
        if position[1] < height - 1 {return Some(Direction::South);}
    }

    if !visited.iter().any(|a| a == &vec![position[0], position[1].saturating_sub(1)]) {
        if position[1] > 1 {return Some(Direction::North);}
    }

    None
}

pub fn backtrack_to_neighboured(grid: &mut Grid, visited: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    for i in 0..grid.width() {
        for j in 0..grid.height() {
            let try_check = visited_with_unvisited(visited, vec![i, j], grid.width(), grid.height());
            
            if try_check.is_some() {
                let dir = try_check.unwrap();
                match dir {
                    Direction::East => grid.erase_wall(i, j, false),
                    Direction::West => grid.erase_wall(i.saturating_sub(1), j, false),
                    Direction::North => grid.erase_wall(i, j.saturating_sub(1), true),
                    _ => grid.erase_wall(i, j, true)
                };
                return match dir {
                    Direction::East => Some(vec![i.saturating_add(1), j]),
                    Direction::West => Some(vec![i.saturating_sub(1), j]),
                    Direction::North => Some(vec![i, j.saturating_sub(1)]),
                    _ => Some(vec![i, j.saturating_add(1)])
                }
            }
        }
    }
    None
}

pub fn algorithm(grid: &mut Grid) -> () {
    let mut visited = Vec::<Vec::<usize>>::new();
    let mut current_cell = vec![0, 0];
    let mut direction: Direction = Direction::East;

    while visited.len() < grid.height() * grid.width() {
        route(grid, &current_cell, &mut direction);

        let new_position = next_cell(&direction, &current_cell);

        let been_here: bool = visited.iter().any(|a| a == &new_position);

        if !been_here {
            match direction {
                Direction::East => grid.erase_wall(current_cell[0], current_cell[1], false),
                Direction::West => grid.erase_wall(current_cell[0]-1, current_cell[1], false),
                Direction::North => grid.erase_wall(current_cell[0], current_cell[1]-1, true),
                _ => grid.erase_wall(current_cell[0], current_cell[1], true)
            };
        
            current_cell = new_position.clone();
        } else {
            let new_stage = backtrack_to_neighboured(grid, &visited); 
            
            if !new_stage.is_some() {
                break;
            }

            current_cell = new_stage.unwrap();
        }

        visited.push(current_cell.clone());
    }
}
