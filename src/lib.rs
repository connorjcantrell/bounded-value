
use rand::Rng;
use std::fmt;

// Contemplating my design choice:

// 1) What are your thoughts on my implementations for the `Range` struct? Would it have been better
// to write an entirely separate trait instead? 
//
// 2) What are your thoughts on `RangeError`? Would you have written a custom error differently? If 
// `RangeError` is acceptable, could it be improved upon? 
//
// 3) I greatly appreciate advice on improving anything else that jumps out at you.


// Reason for building this:

// I assume something like this already exists, but I thought it would be a good exercise. This struct 
// will be used inside a game I am building. I need a way to represent a range that knows its current
// location inside a defined range. This range could represent a players skill for a particular attribute. 
// 
// For instance, I could define an attribute called `confidence` w/ a min = 0, max = 100, and current = 50.
// As the player interacts with game, their confidence can increase or decrease depending on the results
// of an event. 
//
// If you know of a simpler way to achieve this, please share :)

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Range {
    pub min: u16,
    pub max: u16,
    pub current: u16
}

impl Range {
    pub fn new(min: u16, max: u16, current: u16) -> Self {  // -> Result<Range, RangeError>
        // Assert min < max, throw relevant error message
        // Assert current >= min and <= max, throw relevant error message
        Range{ min, max, current }
    }
    
    pub fn increase(mut self) -> Result<u16, RangeError> {
        let value = match self.current.overflowing_add(1) {
            (_, true) => self.max,
            (value, false) => value,
        };
        if value > self.max {
            self.current = self.max;
        } else {
            self.current = value;
        }

        Ok(self.current)
    }

    pub fn decrease(mut self) -> Result<u16, RangeError> {
        let value = match self.current.overflowing_sub(1) {
            (_, true) => self.min,
            (value, false) => value,
        };
        if value < self.min {
            self.current = self.min;
        } else {
            self.current = value;
        }

        Ok(self.current)
    }
    
    pub fn increase_by(mut self, amount: u16) -> Result<u16, RangeError> {
        let value = match self.current.overflowing_add(amount) {
            (_, true) => self.max,
            (value, false) => value,
        };
        if value > self.max {
            self.current = self.max;
        } else {
            self.current = value;
        }

        Ok(self.current)
    }
    
    pub fn decrease_by(mut self, amount: u16) -> Result<u16, RangeError> {
        let value = match self.current.overflowing_sub(amount) {
            (_, true) => self.min,
            (value, false) => value,
        };
        if value < self.min {
            self.current = self.min;
        } else {
            self.current = value;
        }

        Ok(self.current)
    }
    
    pub fn randomize(mut self) -> Result<u16, RangeError> {
        // How is it possible to test an unpredictable result?
        // Am I overlooking a possible error?
        self.current = rand::thread_rng().gen_range(self.min, self.max + 1);
        Ok(self.current)
    }
   
    pub fn maximize(mut self) -> Result<u16, RangeError> {
        // Am I overlooking a possible error?
        self.current = self.max;
        Ok(self.current)
    }

    pub fn minimize(mut self) -> Result<u16, RangeError> {
        // Am I overlooking a possible error?
        self.current = self.min;
        Ok(self.current)
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
