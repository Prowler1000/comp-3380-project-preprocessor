use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Coordinates {
    Geographic {
        longitude: f64,
        latitude: f64,
    }
}

/// Despite fields being named "start" and "end",
/// this struct does not represent a vector, just a line
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Line(pub [Coordinates; 2]);

impl Line {
    pub fn new(a: Coordinates, b: Coordinates) -> Self {
        Self([a, b])
    }

    pub fn a(&self) -> Coordinates {
        self.0[0]
    }

    pub fn b(&self) -> Coordinates {
        self.0[1]
    }
}