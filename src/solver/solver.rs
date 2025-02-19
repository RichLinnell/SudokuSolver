use crate::Grid;
use std::{collections::{HashMap, HashSet}, default, sync::{Arc, Mutex}};
pub struct Solver {
}

#[derive(Default)]
pub struct Cell_Ref{
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct Value_To_Cell_Refs{
    value: i32,
    cell_refs: Vec<Cell_Ref>
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
            Self::set_when_only_one_cell_in_block_supports_value(&thread_grid, &mut value_changed);
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

    fn set_when_only_one_cell_in_block_supports_value(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        // Iterate 9 blocks
        // do a 0..2 cols, and rows from each start of block, and then the logic is just the same as above
        for x in 0..3 {
            for y in 0..3 {
                let mut counts: HashMap<i32, usize> = HashMap::new();
                for x1 in 0..3 {
                    for y1 in 0..3 {
                        let possibilities =
                        {
                            let in_grid = thread_grid.lock().unwrap();
                            let cell = (*in_grid).get_cell(x*3+x1, y*3+y1).unwrap();

                            if cell.get_value() > 0 {
                                continue; // ignore set values 
                            }
                            cell.possibilities().clone() // Clone inside limited scope
                        };
                        for p in possibilities {
                            *counts.entry(p).or_insert(0) += 1;
                        }
                    }
                }
                let unique_numbers: Vec<i32> = counts
                    .iter()
                    .filter(|&(_, &count)| count == 1) // Keep only values with count == 1
                    .map(|(&num, _)| num) // Extract the keys
                    .collect();
                let unique_hash: HashSet<_> = unique_numbers.iter().cloned().collect();

                for x1 in 0..3 { 
                    for y1 in 0..3 {
                        let possibilities =
                        {
                            let in_grid = thread_grid.lock().unwrap();
                            let cell = (*in_grid).get_cell(x*3+x1, y*3+y1).unwrap();
                            cell.possibilities().clone() // Clone inside limited scope
                        };
                        let pos_hash: HashSet<_> = possibilities.iter().cloned().collect();
                        let overlap: Vec<i32> = pos_hash.intersection(&unique_hash).cloned().collect();
                        if (overlap.len() == 1) {
                        let mut in_grid = thread_grid.lock().unwrap();
                        (*in_grid).set_cell(x*3+x1, y*3+y1, overlap[0]);
                        *value_changed = true;
                        }
                    }
                }
            }
        }
    }

    fn reduce_possible_values_for_mutual_pairs_in_block(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        // find the set of cells for each block, put mut refs to them in a Vec, and pass to the common method.
    }

    fn reduce_possible_values_for_mutual_pairs_in_row(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        for row in 0..9
        {
            // pull cells from row into a Vec and pass to common method.
            let mut cells: Vec<Cell_Ref> = Vec::new();
            for col in 0..9
            {
                cells.push(Cell_Ref {
                    x: col,
                    y: row
                });
            }
            *value_changed |= Self::reduce_possible_values_for_mutual_pairs_in_a_nine_cell_set(thread_grid, cells);
        }
    }

    fn reduce_possible_values_for_mutual_pairs_in_col(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool){
        // pull the cells from the col into a Vec and pass to the common method.
    }

    fn reduce_possible_values_for_mutual_pairs_in_a_nine_cell_set(thread_grid: &Arc<Mutex<Grid>>, cells: Vec<Cell_Ref>) -> bool {
        let mut vals_to_cells: HashMap<i32, Cell_Ref> = HashMap::new();
        // allocate sets of Value_To_Cell_Refs for values 1..inc9 for the cells in the block
        for cr in cells
        {
            // deref the grid
            // get the cell poss values
            // assign to hasMap
        }
        // find numbers with only 2 options
        // for this subset, check if any share the same cell_refs
        // for those pair of cells, remove any other options from their possibilities
        true
    }
}