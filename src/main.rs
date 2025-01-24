mod grid;
use grid::Grid; 
use std::io;

fn main() {
    let mut main_grid = Grid::new();
    // prompt user to add starting cells - TODO work out the format for this that works for entry
    let mut cell_value = String::new();
    println!("Enter value x");
    io::stdin().read_line(&mut cell_value).expect("Failed to read input from user.");
    println!("YOu entered {cell_value}");
    let cell_value_int: i32 = cell_value.trim().parse().expect("Failed to parse the value from the entry.");

    // output the starting grid to the screen:
    main_grid.set_cell(0, 0, cell_value_int);
    //   TODO : make a render method for the grid
    main_grid.print_grid();
    // begin main calc cycle
    // use the "solver" methods to solve the grid
    // if we found a solution, output the solution to the screen
    // if we didn't find a solution, output that we didn't find a solution
}
