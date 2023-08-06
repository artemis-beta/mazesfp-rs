use rand::prelude::SliceRandom;
use crate::grid::Grid;
use crate::direction::{next_cell, route};

pub fn cut_path(grid: &mut Grid, path: &Vec<Vec<usize>>, unvisited: &mut Vec<Vec<usize>>) {
    for i in 0..path.len() - 2 {
        let cut_south: bool = path[i+1][1] > path[i][1];
        let cut_north: bool = path[i+1][1] < path[i][1];
        let cut_east: bool = path[i+1][0] > path[i][0];

        if cut_south {grid.erase_wall(path[i][0], path[i][1], true);}
        if cut_north {grid.erase_wall(path[i+1][0], path[i+1][1], true);}
        if cut_east {grid.erase_wall(path[i][0], path[i][1], false);}
        else {grid.erase_wall(path[i+1][0], path[i+1][1], false);}

        match unvisited.iter().position(|a| *a == path[i]) {
            Some(p) => unvisited.remove(p),
            _ => panic!("Cannot find element in unvisited list to remove")
        };
    }
}

pub fn algorithm(grid: &mut Grid) -> () {
    let mut unvisited = Vec::<Vec::<usize>>::new();
    let mut current_path = Vec::<Vec::<usize>>::new();
    let mut current_cell = vec![0, 0];
    current_path.push(current_cell.clone());

    for i in 0..grid.width() {
        for j in 0..grid.height() {
            unvisited.push(vec![i, j]);
        }
    }

    while unvisited.len() > 0 {
        println!("{:?}", unvisited.len());

        let last_element = match current_path.last() {
            Some(p) => p,
            _ => panic!("Failed to retrieve last element of path")
        };

        let direction = route(grid, &last_element);
        let new_position = next_cell(&direction, &last_element);

        let been_here: bool = unvisited.iter().any(|a| a == &new_position);

        if been_here {
            let index = match current_path.iter().position(|a| a == &new_position) {
                Some(p) => p,
                _ => panic!("Failed to get index of entry")
            };

            current_path = current_path[0..index].to_vec();
        } else {
            current_path.push(new_position.clone());
        }

        if new_position == current_cell {
            cut_path(grid, &current_path, &mut unvisited);

            current_cell = match unvisited.choose(&mut rand::thread_rng()) {
                Some(p) => p.to_vec(),
                _ => panic!("Failed to generate randpom cell")
            };
        }
    }
}
