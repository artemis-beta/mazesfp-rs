use std::fmt::Display;

#[derive(Clone)]
pub struct Cell {
    south: bool,
    east: bool
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            south: false,
            east: false
        }
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
        if x >= self.width() {
            panic!("Invalid index {} for grid of width {}", x, self.width());
        }
        if y >= self.height() {
            panic!("Invalid index {} for grid of height {}", y, self.height());
        }
        if south {
            self.cells[y][x].south = true;
        } else {
            self.cells[y][x].east = true;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str: String = "".to_string();
        out_str += format!(
            "{}{}{}", "+", "———+".repeat(self.width()),
            "\n"
        ).as_str();

        for row in self.cells.iter() {
            out_str += "|";
            for cell in row.iter() {
                out_str += "   ";
                out_str += if cell.east {" "} else {"|"}; 
            }
            out_str += "\n";
            for cell in row.iter() {
                out_str += "+";
                out_str += if cell.south {"   "} else {"———"}; 
            }
            out_str += "+\n";
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