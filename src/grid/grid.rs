use eframe::egui::Ui;

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

    pub fn render_grid(&self, ui: &mut Ui){
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
                            egui::Frame::default()
                                .stroke(ui.visuals().widgets.noninteractive.fg_stroke)
                                .outer_margin(egui::Margin{left: left_mgn, right: 0.0, top: 0.0, bottom: 0.0})
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                                .fill(egui::Color32::LIGHT_GRAY)
                                .show(ui, |ui| {
                                    let cell_val = self.get_cell(col, row).unwrap().get_value().to_string(); 
                                    ui.label(
                                        egui::RichText::new(cell_val)
                                        .color(egui::Color32::BLACK)
                                        .size(20.0)
                                        .strong()
                                    );
                                });
                        }
                    })
                });
        }
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