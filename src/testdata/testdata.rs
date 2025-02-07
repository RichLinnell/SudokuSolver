use crate::Grid;
pub struct TestData{
}

impl TestData {
    pub fn set_test_data_simple(grid: &mut Grid) {
        grid.set_cell(0, 0, 3);
        grid.set_cell(5, 0, 2);
        grid.set_cell(6, 0, 9);
        grid.set_cell(7, 0, 6);
        grid.set_cell(0, 1, 1);
        grid.set_cell(1, 1, 4);
        grid.set_cell(6, 1, 2);
        grid.set_cell(8, 1, 8);
        grid.set_cell(2, 2, 2);
        grid.set_cell(3, 2, 9);
        grid.set_cell(4, 2, 7);
        grid.set_cell(5, 2, 1);
        grid.set_cell(2, 3, 1);
        grid.set_cell(3, 3, 4);
        grid.set_cell(7, 3, 2);
        grid.set_cell(8, 3, 6);
        grid.set_cell(0, 4, 2);
        grid.set_cell(2, 4, 4);
        grid.set_cell(4, 4, 8);
        grid.set_cell(5, 4, 5);
        grid.set_cell(7, 4, 9);
        grid.set_cell(0, 5, 7);
        grid.set_cell(2, 5, 8);
        grid.set_cell(4, 5, 1);
        grid.set_cell(8, 5, 3);
        grid.set_cell(0, 6, 4);
        grid.set_cell(1, 6, 5);
        grid.set_cell(2, 6, 3);
        grid.set_cell(7, 6, 1);
        grid.set_cell(1, 7, 1);
        grid.set_cell(3, 7, 7);
        grid.set_cell(4, 7, 5);
        grid.set_cell(5, 7, 4);
        grid.set_cell(2, 8, 7);
        grid.set_cell(3, 8, 1);
        grid.set_cell(5, 8, 9);
        grid.set_cell(6, 8, 6);
        grid.set_cell(8, 8, 4);
    }

    pub fn set_test_data_medium(grid: &mut Grid) {
        grid.set_cell(2, 0, 2);
        grid.set_cell(3, 0, 5);
        grid.set_cell(5, 0, 3);
        grid.set_cell(2, 1, 8);
        grid.set_cell(4, 1, 6);
        grid.set_cell(4, 2, 2);
        grid.set_cell(7, 2, 5);
        grid.set_cell(8, 2, 9);
        grid.set_cell(3, 3, 6);
        grid.set_cell(4, 3, 8);
        grid.set_cell(5, 3, 9);
        grid.set_cell(6, 3, 3);
        grid.set_cell(8, 3, 5);
        grid.set_cell(7, 4, 8);
        grid.set_cell(1, 5, 2);
        grid.set_cell(5, 5, 5);
        grid.set_cell(8, 5, 6);
        grid.set_cell(2, 6, 7);
        grid.set_cell(6, 6, 2);
        grid.set_cell(8, 6, 4);
        grid.set_cell(3, 7, 8);
        grid.set_cell(5, 7, 4);
        grid.set_cell(2, 8, 9);
        grid.set_cell(3, 8, 7);
        grid.set_cell(7, 8, 1);
    }
}