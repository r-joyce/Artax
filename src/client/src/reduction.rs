//use std::fmt;



pub struct Reduction {
    pub sum: u32,
    pub avg: u32,
    pub min: (i32, u32),
    pub max: (i32, u32),
}

impl Reduction {
    pub fn new() -> Reduction {
        Reduction {
            sum: 0,
            avg: 0,
            min: (0, 0),
            max: (0, 0),
        }
    }
}
/*
impl fmt::Debug for Reduction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sum: {}, Avg: {}, Min: {}, Max: {}", self.sum, self.avg, self.min, self.max)
    }
}
*/
