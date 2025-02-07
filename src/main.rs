mod grid;
mod testdata;
mod solver;
use std::{thread, thread::sleep, time::Duration};
use std::sync::{Arc, Mutex};
use eframe::egui;
use grid::Grid;
use testdata::TestData;
use solver::Solver;

// TODO:
// * Move solving into its own module
// * Editing in cell, rather than at bottom
// * Additional "solve" logic
//  * If only cell in row, col or block that can have that value, then set it
//  * Evaluate other solve techniques
// * Handle impossible grids
// * Handle logic fails :
//  * cells with no valid value (possibilities().len == 0)
//  * Cells with data entered that breaches ruleset
// * Indicate when solve is in progress
// * Either handle result objects or remove them

fn main() -> eframe::Result {
    // Set up the main grid
    let mut grid = Grid::new();

    TestData::set_test_data(&mut grid);

    // Form size.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native("Sudoku Solver", options, Box::new(|cc| Ok(Box::new(SudokuApp::new(grid, cc)))))
}

struct SudokuApp {
    pub grid : Arc<Mutex<Grid>>,
    pub edit_values: EditingValues,
}

pub struct EditingValues{
    pub row: i32,
    pub col: i32,
    pub value: i32,
    pub value_as_string: String,
    pub new_value: bool
}

impl EditingValues{
    fn new() -> Self{
        EditingValues { 
            row: 9,
            col: 9,
            value: 0,
            value_as_string: "".to_string(),
            new_value: false,
        }
    }
}

impl SudokuApp {
    fn new(grid:  Grid, cc: &eframe::CreationContext<'_>) -> Self {
        let grid_mut = Arc::new(Mutex::new(grid));
        Self {
            grid: grid_mut,
            edit_values: EditingValues::new(),
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Click a Cell to set its value.");
            self.edit_values.new_value = false;
            let view_grid = Arc::clone(&self.grid);
            (*view_grid.lock().unwrap()).render_grid(ui, &mut self.edit_values);

            ctx.request_repaint_after(Duration::from_millis(200));
            if ui.button("Solve").clicked() {
                let thread_grid = Arc::clone(&self.grid);
                thread::spawn(move || {
                    solver::Solver::solve(thread_grid);
                });
            };
            if self.edit_values.row != 9 {
                if self.edit_values.new_value {
                    self.edit_values.value_as_string = self.edit_values.value.to_string();
                }
                let textbox = ui.text_edit_singleline(&mut self.edit_values.value_as_string);
                if textbox.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Ok(parsed) = self.edit_values.value_as_string.parse::<i32>() {
                        self.edit_values.value = parsed;
                        let view_grid = Arc::clone(&self.grid);
                        (*view_grid.lock().unwrap()).set_cell(self.edit_values.col, self.edit_values.row, self.edit_values.value);
                        self.edit_values.row = 9;
                    }
                }
            }
        });
    }
}