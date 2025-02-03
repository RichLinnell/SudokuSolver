mod grid;
use std::{thread, thread::sleep, time::Duration};
use std::sync::{Arc, Mutex};
use eframe::egui;
use grid::Grid;

// TODO: I've imported eFrame and eGui libraries.  I now need to work out the format of the screen
// I'll be showing the user.  It's going to be a Sudoku grid, and I suspect I want to allow the
// user to move around the cells and add entries.
// Then I will want some buttons at the bottom :
//  - Clear Grid
//  - Solve
//  - Exit
//  We'll also need a way to indicate issues - i.e. "This is an unsolvable Grid" and the like
//  We'll also need a way to indicate duplicate values on the grid - rows with 2 numbers the same
//  and the like.

fn main() -> eframe::Result {
    // Set up the main grid
    let mut grid = Grid::new();

    set_test_data(&mut grid);

    // Form size.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native("Sudoku Solver", options, Box::new(|cc| Ok(Box::new(SudokuApp::new(grid, cc)))))
}

fn set_test_data(grid: &mut Grid) {
    grid.set_cell(3, 4, 8);
    grid.set_cell(6, 2, 1);
    grid.set_cell(5, 8, 6);
    grid.set_cell(3, 1, 9);
    grid.set_cell(0, 5, 2);
    grid.set_cell(7, 7, 8);
}

struct SudokuApp {
    pub grid : Arc<Mutex<Grid>>,
    pub is_solving: bool,
}

impl SudokuApp {
    fn new(grid:  Grid, cc: &eframe::CreationContext<'_>) -> Self {
        let grid_mut = Arc::new(Mutex::new(grid));
        Self {
            grid: grid_mut,
            is_solving: false,
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Click a Cell to set its value.");
            let view_grid = Arc::clone(&self.grid);
            (*view_grid.lock().unwrap()).render_grid(ui);

            if ui.button("Solve").clicked() {
                let thread_grid = Arc::clone(&self.grid);
                thread::spawn(move || {
                    for y in 0..9 {
                        for x in 0..9 {
                            sleep(Duration::from_millis(500));
                            let mut in_grid = thread_grid.lock().unwrap();
                            (*in_grid).set_cell(x, y, 2).unwrap();
                        }
                    }
                });
            }
        });
    }
}