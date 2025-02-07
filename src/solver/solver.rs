use crate::Grid;
use std::sync::{Arc, Mutex};
pub struct Solver {

}

impl Solver {
    pub fn solve(thread_grid: Arc<Mutex<Grid>>){
        let mut value_changed = true;
        while (value_changed){
            value_changed = false;
            for y in 0..9 {
                for x in 0..9 {
                    // sleep(Duration::from_millis(20));
                    let possibilities =
                    {
                        let in_grid = thread_grid.lock().unwrap();
                        let cell = (*in_grid).get_cell(x, y).unwrap();
                        cell.possibilities().clone() // Clone inside limited scope
                    };
                    if possibilities.len() == 1 {
                        let mut in_grid = thread_grid.lock().unwrap();
                        (*in_grid).set_cell(x, y, possibilities[0]);
                        value_changed = true;
                    }
                }
            }
        }
    }
}