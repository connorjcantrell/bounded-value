
use rand::Rng;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct BoundedValue {
    min: u16,
    max: u16,
    value: u16
}

impl BoundedValue {
    pub fn new(min: u16, max: u16, value: u16) -> Self {  // -> Result<BoundedValue, RangeError>
        // Assert min < max, throw relevant error message
        // Assert value >= min and <= max, throw relevant error message
        BoundedValue{ min, max, value }
    }
    
    pub fn increase(&mut self) -> Result<u16, RangeError> {
        let value = match self.value.checked_add(1) {
            Some(value) => value,
            None => self.max,
        };
        self.value = value.min(self.max);
        Ok(self.value)
    }

    pub fn decrease(&mut self) -> Result<u16, RangeError> {
        let value = match self.value.checked_sub(1) {
            Some(value) => value,
            None => self.min,
        };
        self.value = value.min(self.max);
        Ok(self.value)
    }
    
    pub fn increase_by(&mut self, amount: u16) -> Result<u16, RangeError> {
        let value = match self.value.checked_add(amount) {
            Some(value) => value,
            None => self.max,
        };
        self.value = value.min(self.max);
        Ok(self.value)
    }
    
    pub fn decrease_by(&mut self, amount: u16) -> Result<u16, RangeError> {
        let value = match self.value.checked_sub(amount) {
            None => self.min,
            Some(value) => value,
        };
        self.value = value.min(self.max);
        Ok(self.value)
    }
    
    pub fn randomize(&mut self) -> Result<u16, RangeError> {
        // How is it possible to test an unpredictable result?
        // Am I overlooking a possible error?
        self.value = rand::thread_rng().gen_range(self.min, self.max + 1);
        Ok(self.value)
    }
   
    pub fn maximize(&mut self) -> Result<u16, RangeError> {
        // Am I overlooking a possible error?
        self.value = self.max;
        Ok(self.value)
    }

    pub fn minimize(&mut self) -> Result<u16, RangeError> {
        // Am I overlooking a possible error?
        self.value = self.min;
        Ok(self.value)
    }
}

#[derive(Debug, Clone)]
pub struct RangeError {
    pub message: String
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RangeError {
    fn description(&self) -> &str {
        &self.message
    }
}
