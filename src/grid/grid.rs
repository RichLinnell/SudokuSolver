use eframe::egui::Ui;
use egui::was_tooltip_open_last_frame;

use super::cell::Cell;
use crate::EditingValues;

pub struct Grid {
    cells: Vec<Cell>,
}

impl<'a> Grid {
    pub fn new() -> Self {
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
            Some(cell_value) => Ok(cell_value),
            None => Err("No value found".to_string())
        }
    }
    
    pub(crate) fn set_cell(& mut self, x: i32, y: i32, cell_value_int: i32) -> Result<(), String> {
        
        if x<0 || x>8 || y<0 || y>8 {
            return Err("The cell references are outside of the standard 9x9 grid.  Dimensions must be within 0..9:w".to_string());
        }

        // Set the cell value firstly
        let index = (y * 9 + x) as usize;
        let _ = self.cells[index].set_value(cell_value_int);

        // now for every other cell in the row, remove the value from the possible values
        // and for every other cell in the col, remove the value from the possible values
        // Note, remove possuibility will not act on populated cells, so we are safe to call on the cell we set above.
        for c in 0..9 {
                self.remove_possibility(c, y, cell_value_int);
                self.remove_possibility(x, c, cell_value_int);
        }
        // same for the other cells in the block
        // establish block cells - x / 3 * 3 (int division) gives first x cell, same for y. e.g. x=7 (eighth cell) 7 / 3 = 2 * 3 = 6
        let col = x / 3 * 3;
        let row = y / 3 * 3;
        for i in 0..3 {
            self.remove_possibility(col + i, row, cell_value_int);
            self.remove_possibility(col + i, row + 1, cell_value_int);
            self.remove_possibility(col + i, row + 2, cell_value_int);
        }
        return Ok(());
    }

    pub fn remove_possibility(&mut self, x:i32, y: i32, value: i32){
        let index = (y * 9 + x) as usize;
        self.cells[index].remove_possibility(value);
    }
    
    pub fn render_grid(&self, ui: &mut Ui, edit_values: &mut EditingValues){
        for row in 0..9 {
            let mut bottom_pad = 2.0;
            if (row + 1) % 3 == 0 {
                bottom_pad = 15.0;
            }
            egui::Frame::default()
                .outer_margin(egui::Margin{left: 5.0, right: 5.0, top: 0.0, bottom: bottom_pad})
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for col in 0..9 {
                            let mut left_mgn = 0.0;
                            if col % 3 == 0  && col != 0 {
                                left_mgn = 10.0;
                            }
                            let cell_val = self.get_cell(col, row).unwrap().get_value().to_string().replace("0", " "); 
                            let mut bg_color = egui::Color32::GRAY;
                            if cell_val == " " {
                               bg_color = egui::Color32::WHITE; 
                            }
                            egui::Frame::default()
                                .stroke(ui.visuals().widgets.noninteractive.fg_stroke)
                                .outer_margin(egui::Margin{left: left_mgn, right: 0.0, top: 0.0, bottom: 0.0})
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                                .fill(bg_color)
                                .show(ui, |ui| {
                                    ui.set_min_width(12.0);
                                    if(ui.label(
                                        egui::RichText::new(cell_val)
                                        .color(egui::Color32::BLACK)
                                        .size(20.0)
                                        .strong()
                                    )).clicked(){
                                        println!("Call to edit {},{}", col, row);
                                        edit_values.new_value = true;
                                        edit_values.row = row;
                                        edit_values.col = col;
                                        edit_values.value = self.get_cell(col, row).unwrap().get_value();
                                    };
                                });
                        }
                    })
                });
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}