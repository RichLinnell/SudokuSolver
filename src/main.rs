mod grid;
use eframe::egui;
use egui::warn_if_debug_build;
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

    eframe::run_native("Sudoku Solver", options, Box::new(|cc| Ok(Box::new(SudokuApp::new(cc)))))

    // eframe::run_simple_native(
    //     "Sudoku Solving App - Built in Rust.",
    //     options,
    //     move |ctx, _frame| {
    //         egui::CentralPanel::default().show(ctx, |ui| {
    //             ui.heading("Sudoku Solving Application, built in Rust");
    //             ui.horizontal(|ui| {
    //                 let name_label = ui.label("Your name: ");
    //                 ui.text_edit_singleline(&mut name)
    //                     .labelled_by(name_label.id);
    //             });
    //             ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
    //             if ui.button("Increment").clicked() {
    //                 age += 1;
    //             }
    //             ui.label(format!("Hello '{name}', age {age}"));
    //         });
    //     },
    // )
}

#[derive(Default)]
struct SudokuApp{
    // no data
}

impl SudokuApp{
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is the starting point for my Sudoku Solver.");
            for r in [0, 3, 6, 1, 4, 7, 2, 5, 8] {
                ui.horizontal(|ui| {
                    for i in 1..10 {
                        egui::Frame::default()
                            .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                            .fill(egui::Color32::WHITE)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(((i+r)%9).to_string().replace("0", "9"))
                                    .color(egui::Color32::BLACK)
                                    .strong()
                                );
                            });
                    };
                });
            };
        });
    }
}