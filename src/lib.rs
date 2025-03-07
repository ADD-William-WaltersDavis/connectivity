use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Clone)]
pub struct SquareID(pub String);

impl SquareID {
    pub fn new(easting: u32, northing: u32) -> SquareID {
        SquareID(format!("{}_{}", easting, northing))
    }

    pub fn get_xy_as_f64(&self) -> (f64, f64) {
        let x = self.0.split("_").collect::<Vec<&str>>()[0]
            .parse::<f64>()
            .unwrap();
        let y = self.0.split("_").collect::<Vec<&str>>()[1]
            .parse::<f64>()
            .unwrap();
        (x, y)
    }

    pub fn get_xy_as_u32(&self) -> (u32, u32) {
        let x = self.0.split("_").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        let y = self.0.split("_").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        (x, y)
    }
}