use eframe::egui;
use eframe::egui::Ui;
use super::cell::Cell;

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells : std::iter::repeat_with(Cell::new).take(81).collect(),
        }
    }

    /// Parse a puzzle from a string of digits.
    /// Use 0 (or any non-1-9 digit) for blank cells.
    /// All non-digit characters (spaces, newlines, etc.) are ignored.
    ///
    /// Example:
    /// ```text
    /// 000 050 030
    /// 000 704 000
    /// 100 300 049
    /// 000 490 281
    /// 032 580 900
    /// 000 000 000
    /// 965 000 000
    /// 000 000 000
    /// 007 200 000
    /// ```
    pub fn from_string(input: &str) -> Grid {
        let mut grid = Grid::new();
        let digits: Vec<i32> = input.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        for (i, &val) in digits.iter().enumerate().take(81) {
            if val > 0 {
                let x = (i % 9) as i32;
                let y = (i / 9) as i32;
                let _ = grid.set_cell(x, y, val);
            }
        }
        grid
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
            return Err("The cell references are outside of the standard 9x9 grid.  Dimensions must be within 0..9".to_string());
        }

        // Set the cell value firstly
        let index = (y * 9 + x) as usize;
        let _ = self.cells[index].set_value(cell_value_int);

        // now for every other cell in the row, remove the value from the possible values
        // and for every other cell in the col, remove the value from the possible values
        // Note, remove possibility will not act on populated cells, so we are safe to call on the cell we set above.
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

    /// Clear a cell value and recalculate all possibilities from scratch.
    pub fn clear_cell(&mut self, x: i32, y: i32) {
        let index = (y * 9 + x) as usize;
        self.cells[index].clear();
        self.recalculate_possibilities();
    }

    /// Reset all unsolved cells' possibilities and re-propagate constraints
    /// from every solved cell. Called after clearing a cell.
    fn recalculate_possibilities(&mut self) {
        // Reset all unsolved cells to full possibility set
        for cell in &mut self.cells {
            if cell.get_value() == 0 {
                cell.clear();
            }
        }
        // Re-propagate constraints from each solved cell
        for y in 0..9_i32 {
            for x in 0..9_i32 {
                let index = (y * 9 + x) as usize;
                let val = self.cells[index].get_value();
                if val > 0 {
                    for c in 0..9 {
                        self.remove_possibility(c, y, val);
                        self.remove_possibility(x, c, val);
                    }
                    let bx = x / 3 * 3;
                    let by = y / 3 * 3;
                    for dy in 0..3 {
                        for dx in 0..3 {
                            self.remove_possibility(bx + dx, by + dy, val);
                        }
                    }
                }
            }
        }
    }

    pub fn remove_possibility(&mut self, x:i32, y: i32, value: i32) -> bool {
        let index = (y * 9 + x) as usize;
        self.cells[index].remove_possibility(value)
    }

    /// Render the grid. Highlights the selected cell.
    /// Returns the (col, row) of a clicked cell, or None.
    pub fn render_grid(&self, ui: &mut Ui, selected: Option<(i32, i32)>) -> Option<(i32, i32)> {
        let mut clicked: Option<(i32, i32)> = None;
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
                            let is_selected = selected == Some((col, row));
                            let bg_color = if is_selected {
                                egui::Color32::from_rgb(173, 216, 230) // light blue
                            } else if cell_val == " " {
                                egui::Color32::WHITE
                            } else {
                                egui::Color32::GRAY
                            };
                            egui::Frame::default()
                                .stroke(ui.visuals().widgets.noninteractive.fg_stroke)
                                .outer_margin(egui::Margin{left: left_mgn, right: 0.0, top: 0.0, bottom: 0.0})
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                                .fill(bg_color)
                                .show(ui, |ui| {
                                    ui.set_min_width(12.0);
                                    if ui.label(
                                        egui::RichText::new(cell_val)
                                        .color(egui::Color32::BLACK)
                                        .size(20.0)
                                        .strong()
                                    ).clicked() {
                                        clicked = Some((col, row));
                                    };
                                });
                        }
                    })
                });
        }
        clicked
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
