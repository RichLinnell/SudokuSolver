pub struct Cell {
    value: i32,
    possible_values : Vec<i32>
}

impl Cell {
    pub fn new() -> Self {
        Cell { value: 0,
        possible_values: (1..=9).collect(),
        }
    }

    pub fn set_value(&mut self, value: i32) -> Result<(), String>{
        if value > 9 || value < 0 {
            return Err("Cell values cannot be outside of the range 0..9. Use 0 to represent a cell that is empty.".to_string());
        }
        self.value = value;
        self.possible_values = [value].to_vec();
        Ok(())
    }
    
    pub(crate) fn get_value(&self) -> i32 {
        self.value
    }

    pub fn remove_possibility(&mut self, value: i32) {
        if self.value > 0 {
            return;
        }
       self.possible_values.retain(|&x| x != value);
    }

    pub fn possibilities(& self) -> &Vec<i32> {
        &self.possible_values
    }
}