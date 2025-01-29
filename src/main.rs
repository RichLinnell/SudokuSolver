mod grid;
use eframe::egui;
use grid::Grid;
use std::io;

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
    let mut main_grid = Grid::new();

    // this code, as it should move out to another module.
    // Demo form code for the egui framework.
    let mut name = "Test".to_owned();
    let mut age = 55;
    // Form size.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 440.0]),
        ..Default::default()
    };
    eframe::run_simple_native(
        "Sudoku Solving App - Built in Rust.",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Sudoku Solving Application, built in Rust");
                ui.horizontal(|ui| {
                    let name_label = ui.label("Your name: ");
                    ui.text_edit_singleline(&mut name)
                        .labelled_by(name_label.id);
                });
                ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                if ui.button("Increment").clicked() {
                    age += 1;
                }
                ui.label(format!("Hello '{name}', age {age}"));
            });
        },
    )
}
