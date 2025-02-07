use crate::Grid;
use std::{collections::HashMap, collections::HashSet, sync::{Arc, Mutex}};
pub struct Solver {

}

impl Solver {
    pub fn solve(thread_grid: Arc<Mutex<Grid>>){
        let mut iterations = 0;
        let mut value_changed = true;
        while (value_changed){
            value_changed = false;
            Self::set_only_one_possibility_values(&thread_grid, &mut value_changed);
            Self::set_when_only_one_cell_in_row_supports_value(&thread_grid, &mut value_changed);
            Self::set_when_only_one_cell_in_column_supports_value(&thread_grid, &mut value_changed);
            iterations += 1;
        }
        println!("Ending the solve process after {} iteration(s).", iterations);
    }

    fn set_only_one_possibility_values(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        for y in 0..9 {
            for x in 0..9 {
                // sleep(Duration::from_millis(20));
                let mut not_set = true;
                let possibilities =
                {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();

                    if cell.get_value() > 0 {
                        not_set = false;
                    }
                    cell.possibilities().clone() // Clone inside limited scope
                };
                if possibilities.len() == 1 && not_set {
                    let mut in_grid = thread_grid.lock().unwrap();
                    (*in_grid).set_cell(x, y, possibilities[0]);
                    *value_changed = true;
                }
            }
        }
    }   

    fn set_when_only_one_cell_in_column_supports_value(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        for y in 0..9 {
            let mut counts: HashMap<i32, usize> = HashMap::new();
            for x in 0..9 {
                let possibilities =
                {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();

                    if cell.get_value() > 0 {
                       continue; // ignore set values 
                    }
                    cell.possibilities().clone() // Clone inside limited scope
                };
                for p in possibilities {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
            let unique_numbers: Vec<i32> = counts
                .iter()
                .filter(|&(_, &count)| count == 1) // Keep only values with count == 1
                .map(|(&num, _)| num) // Extract the keys
                .collect();
            let unique_hash: HashSet<_> = unique_numbers.iter().cloned().collect();

            for x in 0..9 { 
                let possibilities =
                {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    cell.possibilities().clone() // Clone inside limited scope
                };
                let pos_hash: HashSet<_> = possibilities.iter().cloned().collect();
                let overlap: Vec<i32> = pos_hash.intersection(&unique_hash).cloned().collect();
                if (overlap.len() == 1) {
                    let mut in_grid = thread_grid.lock().unwrap();
                    (*in_grid).set_cell(x, y, overlap[0]);
                    *value_changed = true;
                }
            }
        }
    }

    fn set_when_only_one_cell_in_row_supports_value(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        for y in 0..9 {
            let mut counts: HashMap<i32, usize> = HashMap::new();
            for x in 0..9 {
                let possibilities =
                {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();

                    if cell.get_value() > 0 {
                       continue; // ignore set values 
                    }
                    cell.possibilities().clone() // Clone inside limited scope
                };
                for p in possibilities {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
            let unique_numbers: Vec<i32> = counts
                .iter()
                .filter(|&(_, &count)| count == 1) // Keep only values with count == 1
                .map(|(&num, _)| num) // Extract the keys
                .collect();
            let unique_hash: HashSet<_> = unique_numbers.iter().cloned().collect();

            for x in 0..9 { 
                let possibilities =
                {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    cell.possibilities().clone() // Clone inside limited scope
                };
                let pos_hash: HashSet<_> = possibilities.iter().cloned().collect();
                let overlap: Vec<i32> = pos_hash.intersection(&unique_hash).cloned().collect();
                if (overlap.len() == 1) {
                    let mut in_grid = thread_grid.lock().unwrap();
                    (*in_grid).set_cell(x, y, overlap[0]);
                    *value_changed = true;
                }
            }
        }

    }
}