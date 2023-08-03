use std::fmt::Display;

#[derive(Clone)]
pub struct Cell {
    south: bool,
    east: bool,
    east_wall: char,
    south_wall: char
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            south: false,
            east: false,
            east_wall: '|',
            south_wall: '_'
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if !self.south {self.south_wall} else {' '}, if !self.east {self.east_wall} else {' '})
    }
}

pub struct Grid {
    cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn width(&self) -> usize {
        self.cells[0].len()
    }
    pub fn height(&self) -> usize {
        self.cells.len()
    }
    pub fn erase_wall(&mut self, x: usize, y: usize, south: bool) {
        if south {
            self.cells[y][x].south = true;
        } else {
            self.cells[y][x].east = true;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str: String = " ".to_owned();
        for _ in self.cells.iter() {
            out_str += "_ ";
        }
        out_str += "\n";
        for row in self.cells.iter() {
            out_str += "|";
            for cell in row {
                out_str += format!("{}", cell).as_str();
            }
            out_str += "\n";
        }
        write!(f, "{}", out_str)
    }
}

pub fn build_grid(width: usize, height: usize) -> Grid {
    Grid {
        cells: vec![vec![Cell::default(); width]; height]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_grid() {
        let grid = super::build_grid(10, 10);
        assert!(!grid.cells[0][0].south);
    }
}