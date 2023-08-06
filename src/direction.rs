use rand::{Rng, distributions::{Distribution, Standard}};
use crate::grid::Grid;

#[derive(PartialEq,Debug)]
pub enum Direction {
    North,
    South,
    East,
    West
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::West,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::North
        }
    }
}

pub fn route(grid: &Grid, current_cell: &Vec<usize>) -> Direction {
    loop {
        let direction: Direction = rand::random();

        if current_cell[0] == 0 && direction == Direction::West {
            continue;
        }
        if current_cell[1] == 0 && direction == Direction::North {
            continue;
        }
        if current_cell[0] == grid.width() - 1 && direction == Direction::East {
            continue;
        }
        if current_cell[1] == grid.height() - 1 && direction == Direction::South {
            continue;
        }

        return direction;
    }
}

pub fn next_cell(direction: &Direction, current_cell: &Vec<usize>) -> Vec<usize> {
    let mut new_position = current_cell.clone();
    match direction {
        Direction::East => new_position[0] += 1,
        Direction::West => new_position[0] -= 1,
        Direction::North => new_position[1] -= 1,
        _ => new_position[1] += 1
    }

    return new_position;

}