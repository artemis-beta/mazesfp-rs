use rand::prelude::SliceRandom;
use crate::grid::Grid;
use crate::direction::{next_cell, route, self};

pub fn cut_path(grid: &mut Grid, path: &Vec<Vec<usize>>, unvisited: &mut Vec<Vec<usize>>) {
    for i in 0..path.len() - 1 {
        let cut_south: bool = path[i+1][1] > path[i][1];
        let cut_north: bool = path[i+1][1] < path[i][1];
        let cut_east: bool = path[i+1][0] > path[i][0];
        let cut_west: bool = path[i+1][0] < path[i][0];

        if cut_south {grid.erase_wall(path[i][0], path[i][1], true);}
        if cut_north {grid.erase_wall(path[i+1][0], path[i+1][1], true);}
        if cut_east {grid.erase_wall(path[i][0], path[i][1], false);}
        else if cut_west {grid.erase_wall(path[i+1][0], path[i+1][1], false);}

        let position = unvisited.iter().position(|a| *a == path[i]);

        if position.is_some() {
            unvisited.remove(position.unwrap());
        }
    }
}

pub fn algorithm(grid: &mut Grid) -> () {
    let mut unvisited = Vec::<Vec::<usize>>::new();
    let mut current_path = Vec::<Vec::<usize>>::new();

    for i in 0..grid.width() {
        for j in 0..grid.height() {
            if i == 0 && j == 0 {
                continue;
            }
            unvisited.push(vec![i, j]);
        }
    }

    let start_cell = match unvisited.choose(&mut rand::thread_rng()) {
        Some(p) => p.to_vec(),
        _ => panic!("Failed to generate random cell")
    };

    let mut direction = direction::Direction::East;

    current_path.push(start_cell.clone());

    while unvisited.len() > 0 {
        let last_element = match current_path.last() {
            Some(p) => p,
            _ => panic!("Failed to retrieve last element from {:?}", current_path)
        };

        route(grid, &last_element, &mut direction);

        let new_position = next_cell(&direction, &last_element);

        let been_here: bool = current_path.iter().any(|a| a == &new_position);

        if been_here {
            let index = match current_path.iter().position(|a| a == &new_position) {
                Some(p) => p,
                _ => panic!("Failed to get index of entry")
            };

            let pre_loop_path = current_path[0..index].to_vec();
            if pre_loop_path.len() == 0 {
                let last_position = match current_path.last() {
                    Some(p) => p,
                    _ => panic!("Failed to get last entry")
                };
                current_path = vec![last_position.to_vec()];
            }
            else {
                current_path = pre_loop_path.clone();
            }
        } else {
            current_path.push(new_position.clone());
        }

        if !unvisited.iter().any(|a| a == &new_position) {
            cut_path(grid, &current_path, &mut unvisited);
            current_path.clear();
            let start_cell = unvisited.choose(&mut rand::thread_rng());

            if !start_cell.is_some() {
                break;
            }

            current_path = vec![start_cell.unwrap().to_vec()];
        }
    }
}
