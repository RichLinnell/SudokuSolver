pub struct Cell {
    value: i32,
}

impl Cell {
    pub fn new() -> Self {
        Cell { value: 0 }
    }

    pub fn set_value(&mut self, value: i32) -> Result<(), String>{
        if value > 9 || value < 0 {
            return Err("Cell values cannot be outside of the range 0..9. Use 0 to represent a cell that is empty.".to_string());
        }
        self.value = value;
        Ok(())
    }
    
    pub(crate) fn get_value(&self) -> i32 {
        self.value
    }
}