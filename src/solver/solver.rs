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
            Self::reduce_hidden_triples_in_all_units(&thread_grid, &mut progress);
            Self::reduce_pointing_pairs(&thread_grid, &mut progress);
            Self::reduce_box_line(&thread_grid, &mut progress);
            Self::reduce_x_wing(&thread_grid, &mut progress);
            Self::reduce_swordfish(&thread_grid, &mut progress);
            Self::reduce_y_wing(&thread_grid, &mut progress);

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

    /// Read all 81 cells' possibilities in one lock. Index = y*9+x.
    fn read_all_possibilities(grid: &Arc<Mutex<Grid>>) -> Vec<Vec<i32>> {
        let g = grid.lock().unwrap();
        let mut result = Vec::with_capacity(81);
        for y in 0..9_i32 {
            for x in 0..9_i32 {
                let cell = g.get_cell(x, y).unwrap();
                if cell.get_value() > 0 {
                    result.push(Vec::new());
                } else {
                    result.push(cell.possibilities().clone());
                }
            }
        }
        result
    }

    /// Check whether two cells can see each other (same row, column, or block).
    fn cells_see_each_other(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        if x1 == x2 && y1 == y2 { return false; }
        // Same row
        if y1 == y2 { return true; }
        // Same column
        if x1 == x2 { return true; }
        // Same block
        if x1 / 3 == x2 / 3 && y1 / 3 == y2 / 3 { return true; }
        false
    }

    // =========================================================================
    // Value-setting techniques
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
    // Hidden Triples: if three values in a unit can only appear in the same
    // three cells, all other candidates can be removed from those cells.
    // =========================================================================

    fn reduce_hidden_triples_in_all_units(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        for unit in Self::all_units() {
            *changed |= Self::reduce_hidden_triples_in_unit(grid, &unit);
        }
    }

    fn reduce_hidden_triples_in_unit(grid: &Arc<Mutex<Grid>>, cells: &[CellRef]) -> bool {
        let mut changed = false;
        let cell_poss = Self::read_unit_possibilities(grid, cells);

        // Map each value to the cell indices that can hold it
        let mut value_locations: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, poss) in cell_poss.iter().enumerate() {
            for &v in poss {
                value_locations.entry(v).or_default().push(i);
            }
        }

        // Find values that appear in 2 or 3 cells
        let candidates: Vec<(i32, Vec<usize>)> = value_locations.into_iter()
            .filter(|(_, locs)| locs.len() >= 2 && locs.len() <= 3)
            .collect();

        // Try all combinations of 3 values
        for i in 0..candidates.len() {
            for j in (i + 1)..candidates.len() {
                for k in (j + 1)..candidates.len() {
                    let cell_union: HashSet<usize> = candidates[i].1.iter()
                        .chain(candidates[j].1.iter())
                        .chain(candidates[k].1.iter())
                        .cloned().collect();

                    if cell_union.len() == 3 {
                        let v1 = candidates[i].0;
                        let v2 = candidates[j].0;
                        let v3 = candidates[k].0;
                        // Remove all other candidates from these 3 cells
                        let mut g = grid.lock().unwrap();
                        for &idx in &cell_union {
                            let cr = &cells[idx];
                            for &v in &cell_poss[idx] {
                                if v != v1 && v != v2 && v != v3 {
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

    // =========================================================================
    // X-Wing: if a candidate appears in exactly 2 cells in each of 2 rows,
    // and those cells share the same 2 columns, eliminate it from other cells
    // in those columns (and vice versa for columns → rows).
    // =========================================================================

    fn reduce_x_wing(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        let all_poss = Self::read_all_possibilities(grid);

        for val in 1..=9_i32 {
            // Row-based X-Wing
            // For each row, find which columns hold this candidate
            let mut row_cols: Vec<Vec<i32>> = Vec::new();
            for row in 0..9_i32 {
                let cols: Vec<i32> = (0..9_i32)
                    .filter(|&col| all_poss[(row * 9 + col) as usize].contains(&val))
                    .collect();
                row_cols.push(cols);
            }

            for r1 in 0..9_usize {
                if row_cols[r1].len() != 2 { continue; }
                for r2 in (r1 + 1)..9 {
                    if row_cols[r2].len() != 2 { continue; }
                    if row_cols[r1] == row_cols[r2] {
                        // X-Wing found: eliminate val from these 2 columns in other rows
                        let c1 = row_cols[r1][0];
                        let c2 = row_cols[r1][1];
                        let mut g = grid.lock().unwrap();
                        for row in 0..9_i32 {
                            if row != r1 as i32 && row != r2 as i32 {
                                *changed |= g.remove_possibility(c1, row, val);
                                *changed |= g.remove_possibility(c2, row, val);
                            }
                        }
                    }
                }
            }

            // Column-based X-Wing
            // For each column, find which rows hold this candidate
            let mut col_rows: Vec<Vec<i32>> = Vec::new();
            for col in 0..9_i32 {
                let rows: Vec<i32> = (0..9_i32)
                    .filter(|&row| all_poss[(row * 9 + col) as usize].contains(&val))
                    .collect();
                col_rows.push(rows);
            }

            for c1 in 0..9_usize {
                if col_rows[c1].len() != 2 { continue; }
                for c2 in (c1 + 1)..9 {
                    if col_rows[c2].len() != 2 { continue; }
                    if col_rows[c1] == col_rows[c2] {
                        // X-Wing found: eliminate val from these 2 rows in other columns
                        let r1 = col_rows[c1][0];
                        let r2 = col_rows[c1][1];
                        let mut g = grid.lock().unwrap();
                        for col in 0..9_i32 {
                            if col != c1 as i32 && col != c2 as i32 {
                                *changed |= g.remove_possibility(col, r1, val);
                                *changed |= g.remove_possibility(col, r2, val);
                            }
                        }
                    }
                }
            }
        }
    }

    // =========================================================================
    // Swordfish: extension of X-Wing to 3 rows/columns. If a candidate
    // appears in 2-3 positions in each of 3 rows, and all those positions
    // fall within the same 3 columns, eliminate it from other cells in
    // those columns (and vice versa).
    // =========================================================================

    fn reduce_swordfish(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        let all_poss = Self::read_all_possibilities(grid);

        for val in 1..=9_i32 {
            // Row-based Swordfish
            let mut row_cols: Vec<(i32, Vec<i32>)> = Vec::new();
            for row in 0..9_i32 {
                let cols: Vec<i32> = (0..9_i32)
                    .filter(|&col| all_poss[(row * 9 + col) as usize].contains(&val))
                    .collect();
                if cols.len() >= 2 && cols.len() <= 3 {
                    row_cols.push((row, cols));
                }
            }

            for i in 0..row_cols.len() {
                for j in (i + 1)..row_cols.len() {
                    for k in (j + 1)..row_cols.len() {
                        let col_union: HashSet<i32> = row_cols[i].1.iter()
                            .chain(row_cols[j].1.iter())
                            .chain(row_cols[k].1.iter())
                            .cloned().collect();

                        if col_union.len() == 3 {
                            let rows = [row_cols[i].0, row_cols[j].0, row_cols[k].0];
                            let mut g = grid.lock().unwrap();
                            for &col in &col_union {
                                for row in 0..9_i32 {
                                    if !rows.contains(&row) {
                                        *changed |= g.remove_possibility(col, row, val);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Column-based Swordfish
            let mut col_rows: Vec<(i32, Vec<i32>)> = Vec::new();
            for col in 0..9_i32 {
                let rows: Vec<i32> = (0..9_i32)
                    .filter(|&row| all_poss[(row * 9 + col) as usize].contains(&val))
                    .collect();
                if rows.len() >= 2 && rows.len() <= 3 {
                    col_rows.push((col, rows));
                }
            }

            for i in 0..col_rows.len() {
                for j in (i + 1)..col_rows.len() {
                    for k in (j + 1)..col_rows.len() {
                        let row_union: HashSet<i32> = col_rows[i].1.iter()
                            .chain(col_rows[j].1.iter())
                            .chain(col_rows[k].1.iter())
                            .cloned().collect();

                        if row_union.len() == 3 {
                            let cols = [col_rows[i].0, col_rows[j].0, col_rows[k].0];
                            let mut g = grid.lock().unwrap();
                            for &row in &row_union {
                                for col in 0..9_i32 {
                                    if !cols.contains(&col) {
                                        *changed |= g.remove_possibility(col, row, val);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // =========================================================================
    // Y-Wing (XY-Wing): a pivot cell with candidates {A,B} connects to two
    // wing cells — one with {A,C} and one with {B,C}. Any cell that sees
    // both wings cannot contain C.
    // =========================================================================

    fn reduce_y_wing(grid: &Arc<Mutex<Grid>>, changed: &mut bool) {
        let all_poss = Self::read_all_possibilities(grid);

        // Collect all bivalue cells (exactly 2 candidates)
        let bivalue: Vec<(i32, i32, Vec<i32>)> = all_poss.iter().enumerate()
            .filter(|(_, poss)| poss.len() == 2)
            .map(|(idx, poss)| {
                let x = (idx % 9) as i32;
                let y = (idx / 9) as i32;
                let mut sorted = poss.clone();
                sorted.sort();
                (x, y, sorted)
            })
            .collect();

        // For each potential pivot
        for pivot in &bivalue {
            let (px, py, pvals) = pivot;
            let a = pvals[0];
            let b = pvals[1];

            // Find wings that see the pivot
            let mut wings_ac: Vec<&(i32, i32, Vec<i32>)> = Vec::new(); // contain A but not B
            let mut wings_bc: Vec<&(i32, i32, Vec<i32>)> = Vec::new(); // contain B but not A

            for wing in &bivalue {
                let (wx, wy, wvals) = wing;
                if wx == px && wy == py { continue; }
                if !Self::cells_see_each_other(*px, *py, *wx, *wy) { continue; }

                if wvals.contains(&a) && !wvals.contains(&b) {
                    wings_ac.push(wing);
                }
                if wvals.contains(&b) && !wvals.contains(&a) {
                    wings_bc.push(wing);
                }
            }

            // For each pair of wings (one AC, one BC)
            for wa in &wings_ac {
                for wb in &wings_bc {
                    // Find C: the shared value between the two wings that isn't A or B
                    let c_candidates: Vec<i32> = wa.2.iter()
                        .filter(|&&v| v != a && wb.2.contains(&v))
                        .cloned().collect();
                    if c_candidates.len() != 1 { continue; }
                    let c = c_candidates[0];

                    // Eliminate C from all cells that see BOTH wings
                    let mut g = grid.lock().unwrap();
                    for y in 0..9_i32 {
                        for x in 0..9_i32 {
                            if (x == wa.0 && y == wa.1) || (x == wb.0 && y == wb.1) { continue; }
                            if x == *px && y == *py { continue; }
                            if Self::cells_see_each_other(x, y, wa.0, wa.1)
                                && Self::cells_see_each_other(x, y, wb.0, wb.1) {
                                *changed |= g.remove_possibility(x, y, c);
                            }
                        }
                    }
                }
            }
        }
    }
}
