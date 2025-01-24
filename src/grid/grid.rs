use std::result;

use super::cell::Cell;

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
        cells : std::iter::repeat_with(Cell::new).take(81).collect(),
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Result<&Cell, String> {
        if x<0 || x>8 || y<0 || y>8 {
            return Err("Outside bounds of the grid.".to_string());
        }
        let index = (y * 9 + x) as usize;
        match self.cells.get(index){
            Some(&ref cell_value) => Ok(&cell_value),
            None => Err("No value found".to_string())
        }
    }
    
    pub(crate) fn set_cell(& mut self, x: i32, y: i32, cell_value_int: i32) -> Result<(), String> {
        
        if x<0 || x>8 || y<0 || y>8 {
            return Err("The cell references are outside of the standard 9x9 grid.  Dimensions must be within 0..9:w".to_string());
        }

        let index = (y * 9 + x) as usize;
        self.cells[index].set_value(cell_value_int);
        Ok(())
    }

    pub fn print_grid(&self) -> Result<(), String> {
        println!("-------------------------------------");
        for y in 0..9 {
            for x in 0..9 {
                let cell_value = self.get_cell(x, y)?.get_value();
                let mut print_str = " ".to_string();
                if cell_value > 0 {
                    print_str = cell_value.to_string();
                }
                print!("| {} ", print_str);
            }
            println!("|");
            println!("-------------------------------------");
        }
        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}