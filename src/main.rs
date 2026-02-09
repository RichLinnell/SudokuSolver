mod grid;
mod testdata;
mod solver;
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use eframe::egui;
use grid::Grid;
use testdata::TestData;
use solver::Solver;

// TODO:
// * Handle impossible grids
// * Handle logic fails :
//  * cells with no valid value (possibilities().len == 0)
//  * Cells with data entered that breaches ruleset
// * Indicate when solve is in progress
// * Either handle result objects or remove them

fn main() -> eframe::Result {
    // Change TestData::expert() to simple(), medium(), or hard() to load a different puzzle.
    let grid = TestData::expert();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([390.0, 500.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native("Sudoku Solver", options, Box::new(|_cc| Ok(Box::new(SudokuApp::new(grid)))))
}

struct SudokuApp {
    pub grid: Arc<Mutex<Grid>>,
    pub selected_cell: Option<(i32, i32)>,
    pub status: Arc<Mutex<String>>,
}

impl SudokuApp {
    fn new(grid: Grid) -> Self {
        Self {
            grid: Arc::new(Mutex::new(grid)),
            selected_cell: None,
            status: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Click a cell, type 1-9. Delete to clear.");

            // Compute conflicts for display
            let conflicts = {
                let grid = self.grid.lock().unwrap();
                grid.find_conflicts()
            };

            // Render grid — returns which cell was clicked (if any)
            let clicked = {
                let grid = self.grid.lock().unwrap();
                grid.render_grid(ui, self.selected_cell, &conflicts)
            };

            if let Some(cell) = clicked {
                self.selected_cell = Some(cell);
            }

            // Handle keyboard input for the selected cell
            if let Some((col, row)) = self.selected_cell {
                let keys = [
                    (egui::Key::Num1, 1), (egui::Key::Num2, 2), (egui::Key::Num3, 3),
                    (egui::Key::Num4, 4), (egui::Key::Num5, 5), (egui::Key::Num6, 6),
                    (egui::Key::Num7, 7), (egui::Key::Num8, 8), (egui::Key::Num9, 9),
                ];
                for &(key, num) in &keys {
                    if ui.input(|i| i.key_pressed(key)) {
                        let mut grid = self.grid.lock().unwrap();
                        let _ = grid.set_cell(col, row, num);
                        grid.mark_user_entered(col, row);
                        // Clear status on user edit
                        let mut s = self.status.lock().unwrap();
                        *s = String::new();
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
                    let mut grid = self.grid.lock().unwrap();
                    grid.clear_cell(col, row);
                    let mut s = self.status.lock().unwrap();
                    *s = String::new();
                }
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.selected_cell = None;
                }
                // Arrow key navigation
                if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                    self.selected_cell = Some(((col + 1).min(8), row));
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                    self.selected_cell = Some(((col - 1).max(0), row));
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    self.selected_cell = Some((col, (row + 1).min(8)));
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    self.selected_cell = Some((col, (row - 1).max(0)));
                }
            }

            ctx.request_repaint_after(Duration::from_millis(200));

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let solve_btn = egui::Button::new(
                    egui::RichText::new("Solve")
                        .size(15.0)
                        .strong()
                        .color(egui::Color32::WHITE)
                )
                .fill(egui::Color32::from_rgb(70, 130, 180))
                .min_size(egui::vec2(160.0, 34.0))
                .rounding(6.0);

                if ui.add(solve_btn).clicked() {
                    let thread_grid = Arc::clone(&self.grid);
                    let thread_status = Arc::clone(&self.status);
                    thread::spawn(move || {
                        Solver::solve(thread_grid, thread_status);
                    });
                }

                ui.add_space(8.0);

                let clear_btn = egui::Button::new(
                    egui::RichText::new("Clear Grid")
                        .size(15.0)
                        .strong()
                        .color(egui::Color32::WHITE)
                )
                .fill(egui::Color32::from_rgb(180, 100, 100))
                .min_size(egui::vec2(160.0, 34.0))
                .rounding(6.0);

                if ui.add(clear_btn).clicked() {
                    let mut grid = self.grid.lock().unwrap();
                    grid.clear_all();
                    self.selected_cell = None;
                    let mut s = self.status.lock().unwrap();
                    *s = String::new();
                }
            });

            // Display status message
            let status_text = {
                let s = self.status.lock().unwrap();
                s.clone()
            };
            if !status_text.is_empty() {
                ui.add_space(6.0);
                let (color, text) = if status_text.contains("unsolvable") || status_text.contains("conflicting") {
                    (egui::Color32::from_rgb(200, 50, 50), &status_text)
                } else if status_text.contains("brute force...") {
                    (egui::Color32::from_rgb(200, 150, 50), &status_text)
                } else if status_text.contains("Solved") || status_text.contains("solved") {
                    (egui::Color32::from_rgb(50, 160, 50), &status_text)
                } else {
                    (egui::Color32::from_rgb(70, 130, 180), &status_text)
                };
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(text.as_str())
                            .color(color)
                            .size(14.0)
                            .strong()
                    );
                });
            }
        });
    }
}
