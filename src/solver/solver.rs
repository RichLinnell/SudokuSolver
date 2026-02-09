use crate::Grid;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct Solver {
}

#[derive(Default, Clone)]
struct CellRef {
    x: i32,
    y: i32,
}

impl Solver {
    pub fn solve(thread_grid: Arc<Mutex<Grid>>) {
        let mut iterations = 0;
        let mut progress = true;
        while progress {
            progress = false;

            // Candidate elimination techniques (reduce possibilities without setting values)
            Self::reduce_naked_pairs_in_all_units(&thread_grid, &mut progress);
            Self::reduce_naked_triples_in_all_units(&thread_grid, &mut progress);
            Self::reduce_hidden_pairs_in_all_units(&thread_grid, &mut progress);
            Self::reduce_pointing_pairs(&thread_grid, &mut progress);
            Self::reduce_box_line(&thread_grid, &mut progress);

            // Value-setting techniques
            Self::set_only_one_possibility_values(&thread_grid, &mut progress);
            Self::set_when_only_one_cell_in_row_supports_value(&thread_grid, &mut progress);
            Self::set_when_only_one_cell_in_column_supports_value(&thread_grid, &mut progress);
            Self::set_when_only_one_cell_in_block_supports_value(&thread_grid, &mut progress);

            iterations += 1;
        }
        println!("Ending the solve process after {} iteration(s).", iterations);
    }

    // --- Helper methods for building cell reference sets ---

    fn row_cells(row: i32) -> Vec<CellRef> {
        (0..9).map(|col| CellRef { x: col, y: row }).collect()
    }

    fn col_cells(col: i32) -> Vec<CellRef> {
        (0..9).map(|row| CellRef { x: col, y: row }).collect()
    }

    fn block_cells(bx: i32, by: i32) -> Vec<CellRef> {
        let mut cells = Vec::new();
        for dy in 0..3 {
            for dx in 0..3 {
                cells.push(CellRef { x: bx * 3 + dx, y: by * 3 + dy });
            }
        }
        cells
    }

    fn all_units() -> Vec<Vec<CellRef>> {
        let mut units = Vec::new();
        for i in 0..9 {
            units.push(Self::row_cells(i));
            units.push(Self::col_cells(i));
        }
        for bx in 0..3 {
            for by in 0..3 {
                units.push(Self::block_cells(bx, by));
            }
        }
        units
    }

    /// Read possibilities for all cells in a unit in a single lock.
    /// Returns empty Vec for solved cells.
    fn read_unit_possibilities(grid: &Arc<Mutex<Grid>>, cells: &[CellRef]) -> Vec<Vec<i32>> {
        let g = grid.lock().unwrap();
        cells.iter().map(|cr| {
            let cell = g.get_cell(cr.x, cr.y).unwrap();
            if cell.get_value() > 0 {
                Vec::new()
            } else {
                cell.possibilities().clone()
            }
        }).collect()
    }

    // =========================================================================
    // Existing value-setting techniques
    // =========================================================================

    /// Naked singles: if a cell has only one possibility left, set it.
    fn set_only_one_possibility_values(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool) {
        for y in 0..9 {
            for x in 0..9 {
                let mut not_set = true;
                let possibilities = {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    if cell.get_value() > 0 {
                        not_set = false;
                    }
                    cell.possibilities().clone()
                };
                if possibilities.len() == 1 && not_set {
                    let mut in_grid = thread_grid.lock().unwrap();
                    let _ = (*in_grid).set_cell(x, y, possibilities[0]);
                    *value_changed = true;
                }
            }
        }
    }

    /// Hidden singles in rows: if only one cell in a row can hold a value, set it.
    fn set_when_only_one_cell_in_row_supports_value(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool) {
        for y in 0..9 {
            let mut counts: HashMap<i32, usize> = HashMap::new();
            for x in 0..9 {
                let possibilities = {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    if cell.get_value() > 0 {
                       continue;
                    }
                    cell.possibilities().clone()
                };
                for p in possibilities {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
            let unique_numbers: Vec<i32> = counts
                .iter()
                .filter(|&(_, &count)| count == 1)
                .map(|(&num, _)| num)
                .collect();
            let unique_hash: HashSet<_> = unique_numbers.iter().cloned().collect();

            for x in 0..9 {
                let possibilities = {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    cell.possibilities().clone()
                };
                let pos_hash: HashSet<_> = possibilities.iter().cloned().collect();
                let overlap: Vec<i32> = pos_hash.intersection(&unique_hash).cloned().collect();
                if overlap.len() == 1 {
                    let mut in_grid = thread_grid.lock().unwrap();
                    let _ = (*in_grid).set_cell(x, y, overlap[0]);
                    *value_changed = true;
                }
            }
        }
    }

    /// Hidden singles in columns: if only one cell in a column can hold a value, set it.
    fn set_when_only_one_cell_in_column_supports_value(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool) {
        for x in 0..9 {
            let mut counts: HashMap<i32, usize> = HashMap::new();
            for y in 0..9 {
                let possibilities = {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    if cell.get_value() > 0 {
                       continue;
                    }
                    cell.possibilities().clone()
                };
                for p in possibilities {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
            let unique_numbers: Vec<i32> = counts
                .iter()
                .filter(|&(_, &count)| count == 1)
                .map(|(&num, _)| num)
                .collect();
            let unique_hash: HashSet<_> = unique_numbers.iter().cloned().collect();

            for y in 0..9 {
                let possibilities = {
                    let in_grid = thread_grid.lock().unwrap();
                    let cell = (*in_grid).get_cell(x, y).unwrap();
                    cell.possibilities().clone()
                };
                let pos_hash: HashSet<_> = possibilities.iter().cloned().collect();
                let overlap: Vec<i32> = pos_hash.intersection(&unique_hash).cloned().collect();
                if overlap.len() == 1 {
                    let mut in_grid = thread_grid.lock().unwrap();
                    let _ = (*in_grid).set_cell(x, y, overlap[0]);
                    *value_changed = true;
                }
            }
        }
    }

    /// Hidden singles in blocks: if only one cell in a 3x3 block can hold a value, set it.
    fn set_when_only_one_cell_in_block_supports_value(thread_grid: &Arc<Mutex<Grid>>, value_changed: &mut bool) {
        for x in 0..3 {
            for y in 0..3 {
                let mut counts: HashMap<i32, usize> = HashMap::new();
                for x1 in 0..3 {
                    for y1 in 0..3 {
                        let possibilities = {
                            let in_grid = thread_grid.lock().unwrap();
                            let cell = (*in_grid).get_cell(x*3+x1, y*3+y1).unwrap();
                            if cell.get_value() > 0 {
                                continue;
                            }
                            cell.possibilities().clone()
                        };
                        for p in possibilities {
                            *counts.entry(p).or_insert(0) += 1;
                        }
                    }
                }
                let unique_numbers: Vec<i32> = counts
                    .iter()
                    .filter(|&(_, &count)| count == 1)
                    .map(|(&num, _)| num)
                    .collect();
                let unique_hash: HashSet<_> = unique_numbers.iter().cloned().collect();

                for x1 in 0..3 {
                    for y1 in 0..3 {
                        let possibilities = {
                            let in_grid = thread_grid.lock().unwrap();
                            let cell = (*in_grid).get_cell(x*3+x1, y*3+y1).unwrap();
                            cell.possibilities().clone()
                        };
                        let pos_hash: HashSet<_> = possibilities.iter().cloned().collect();
                        let overlap: Vec<i32> = pos_hash.intersection(&unique_hash).cloned().collect();
                        if overlap.len() == 1 {
                            let mut in_grid = thread_grid.lock().unwrap();
                            let _ = (*in_grid).set_cell(x*3+x1, y*3+y1, overlap[0]);
                            *value_changed = true;
                        }
                    }
                }
            }
        }
    }

    // =========================================================================
    // Naked Pairs: if two cells in a unit share the same two candidates,
    // those values can be removed from all other cells in the unit.
    // =========================================================================

    fn reduce_naked_pairs_in_all_units(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        for unit in Self::all_units() {
            *changed |= Self::reduce_naked_pairs_in_unit(grid, &unit);
        }
    }

    fn reduce_naked_pairs_in_unit(grid: &Arc<Mutex<Grid>>, cells: &[CellRef]) -> bool {
        let mut changed = false;
        let cell_poss = Self::read_unit_possibilities(grid, cells);

        // Find cells with exactly 2 possibilities
        let pairs: Vec<(usize, &Vec<i32>)> = cell_poss.iter().enumerate()
            .filter(|(_, poss)| poss.len() == 2)
            .collect();

        // Check for matching pairs
        for i in 0..pairs.len() {
            for j in (i + 1)..pairs.len() {
                let mut a = pairs[i].1.clone();
                let mut b = pairs[j].1.clone();
                a.sort();
                b.sort();
                if a == b {
                    let v1 = a[0];
                    let v2 = a[1];
                    let idx_a = pairs[i].0;
                    let idx_b = pairs[j].0;
                    // Remove these values from all other cells in the unit
                    let mut g = grid.lock().unwrap();
                    for (k, cr) in cells.iter().enumerate() {
                        if k != idx_a && k != idx_b {
                            changed |= g.remove_possibility(cr.x, cr.y, v1);
                            changed |= g.remove_possibility(cr.x, cr.y, v2);
                        }
                    }
                }
            }
        }
        changed
    }

    // =========================================================================
    // Naked Triples: if three cells in a unit have candidates that are a subset
    // of exactly three values, those values can be removed from other cells.
    // =========================================================================

    fn reduce_naked_triples_in_all_units(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        for unit in Self::all_units() {
            *changed |= Self::reduce_naked_triples_in_unit(grid, &unit);
        }
    }

    fn reduce_naked_triples_in_unit(grid: &Arc<Mutex<Grid>>, cells: &[CellRef]) -> bool {
        let mut changed = false;
        let cell_poss = Self::read_unit_possibilities(grid, cells);

        // Collect unsolved cells with 2 or 3 possibilities
        let candidates: Vec<(usize, HashSet<i32>)> = cell_poss.iter().enumerate()
            .filter(|(_, poss)| poss.len() == 2 || poss.len() == 3)
            .map(|(i, poss)| (i, poss.iter().cloned().collect()))
            .collect();

        // Try all combinations of 3
        for i in 0..candidates.len() {
            for j in (i + 1)..candidates.len() {
                for k in (j + 1)..candidates.len() {
                    let union: HashSet<i32> = candidates[i].1
                        .union(&candidates[j].1).cloned().collect::<HashSet<i32>>()
                        .union(&candidates[k].1).cloned().collect();
                    if union.len() == 3 {
                        let idx_a = candidates[i].0;
                        let idx_b = candidates[j].0;
                        let idx_c = candidates[k].0;
                        let mut g = grid.lock().unwrap();
                        for (m, cr) in cells.iter().enumerate() {
                            if m != idx_a && m != idx_b && m != idx_c {
                                for &v in &union {
                                    changed |= g.remove_possibility(cr.x, cr.y, v);
                                }
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    // =========================================================================
    // Hidden Pairs: if two values in a unit can only appear in the same two
    // cells, all other candidates can be removed from those two cells.
    // =========================================================================

    fn reduce_hidden_pairs_in_all_units(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        for unit in Self::all_units() {
            *changed |= Self::reduce_hidden_pairs_in_unit(grid, &unit);
        }
    }

    fn reduce_hidden_pairs_in_unit(grid: &Arc<Mutex<Grid>>, cells: &[CellRef]) -> bool {
        let mut changed = false;
        let cell_poss = Self::read_unit_possibilities(grid, cells);

        // Map each value to the cell indices that can hold it
        let mut value_locations: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, poss) in cell_poss.iter().enumerate() {
            for &v in poss {
                value_locations.entry(v).or_default().push(i);
            }
        }

        // Find values that appear in exactly 2 cells
        let pair_values: Vec<(i32, Vec<usize>)> = value_locations.into_iter()
            .filter(|(_, locs)| locs.len() == 2)
            .collect();

        // Check if any two such values share the same two cells
        for i in 0..pair_values.len() {
            for j in (i + 1)..pair_values.len() {
                if pair_values[i].1 == pair_values[j].1 {
                    let v1 = pair_values[i].0;
                    let v2 = pair_values[j].0;
                    let cell_a = pair_values[i].1[0];
                    let cell_b = pair_values[i].1[1];
                    // Remove all other candidates from these two cells
                    let mut g = grid.lock().unwrap();
                    for &idx in &[cell_a, cell_b] {
                        let cr = &cells[idx];
                        for &v in &cell_poss[idx] {
                            if v != v1 && v != v2 {
                                changed |= g.remove_possibility(cr.x, cr.y, v);
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    // =========================================================================
    // Pointing Pairs: if a candidate in a block only appears in one row or
    // column, it can be eliminated from that row/column outside the block.
    // =========================================================================

    fn reduce_pointing_pairs(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        for bx in 0..3_i32 {
            for by in 0..3_i32 {
                let block = Self::block_cells(bx, by);
                let block_poss = Self::read_unit_possibilities(grid, &block);

                for val in 1..=9 {
                    let mut rows_with_val: HashSet<i32> = HashSet::new();
                    let mut cols_with_val: HashSet<i32> = HashSet::new();
                    let mut count = 0;

                    for (i, poss) in block_poss.iter().enumerate() {
                        if poss.contains(&val) {
                            rows_with_val.insert(block[i].y);
                            cols_with_val.insert(block[i].x);
                            count += 1;
                        }
                    }

                    if count < 2 { continue; }

                    // All occurrences in the same row — eliminate from rest of that row
                    if rows_with_val.len() == 1 {
                        let row = *rows_with_val.iter().next().unwrap();
                        let mut g = grid.lock().unwrap();
                        for x in 0..9 {
                            if x / 3 != bx {
                                *changed |= g.remove_possibility(x, row, val);
                            }
                        }
                    }

                    // All occurrences in the same column — eliminate from rest of that column
                    if cols_with_val.len() == 1 {
                        let col = *cols_with_val.iter().next().unwrap();
                        let mut g = grid.lock().unwrap();
                        for y in 0..9 {
                            if y / 3 != by {
                                *changed |= g.remove_possibility(col, y, val);
                            }
                        }
                    }
                }
            }
        }
    }

    // =========================================================================
    // Box/Line Reduction: if a candidate in a row/column only appears within
    // one block, it can be eliminated from other cells in that block.
    // =========================================================================

    fn reduce_box_line(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        // Row-based: value in a row confined to one block
        for row in 0..9_i32 {
            let row_cells = Self::row_cells(row);
            let row_poss = Self::read_unit_possibilities(grid, &row_cells);

            for val in 1..=9 {
                let mut blocks: HashSet<i32> = HashSet::new();
                let mut count = 0;
                for (i, poss) in row_poss.iter().enumerate() {
                    if poss.contains(&val) {
                        blocks.insert(row_cells[i].x / 3);
                        count += 1;
                    }
                }
                if count < 2 { continue; }
                if blocks.len() == 1 {
                    let bx = *blocks.iter().next().unwrap();
                    let by = row / 3;
                    let mut g = grid.lock().unwrap();
                    for dy in 0..3 {
                        let y = by * 3 + dy;
                        if y != row {
                            for dx in 0..3 {
                                let x = bx * 3 + dx;
                                *changed |= g.remove_possibility(x, y, val);
                            }
                        }
                    }
                }
            }
        }

        // Column-based: value in a column confined to one block
        for col in 0..9_i32 {
            let col_cells = Self::col_cells(col);
            let col_poss = Self::read_unit_possibilities(grid, &col_cells);

            for val in 1..=9 {
                let mut blocks: HashSet<i32> = HashSet::new();
                let mut count = 0;
                for (i, poss) in col_poss.iter().enumerate() {
                    if poss.contains(&val) {
                        blocks.insert(col_cells[i].y / 3);
                        count += 1;
                    }
                }
                if count < 2 { continue; }
                if blocks.len() == 1 {
                    let by = *blocks.iter().next().unwrap();
                    let bx = col / 3;
                    let mut g = grid.lock().unwrap();
                    for dx in 0..3 {
                        let x = bx * 3 + dx;
                        if x != col {
                            for dy in 0..3 {
                                let y = by * 3 + dy;
                                *changed |= g.remove_possibility(x, y, val);
                            }
                        }
                    }
                }
            }
        }
    }
}
