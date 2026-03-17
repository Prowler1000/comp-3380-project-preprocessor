pub mod polygon;

use serde::{Deserialize, Serialize};

const EARTH_RADIUS: f64 = 6_371_000.0; // meters
const STANDARD_PARALLEL: f64 = 49.869916; // Roughly middle of Winnipeg
const CENTRAL_PARALLEL: f64 = STANDARD_PARALLEL; // Just make them the same
const CENTRAL_MERIDIAN: f64 = -97.142445; // Roughly middle of Winnipeg

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

impl Coordinates {
    pub fn project_geographic(&self, standard_parallel: f64, central_parallel: f64, central_meridian: f64) -> (f64, f64) {
        let standard_parallel = standard_parallel.to_radians();
        let central_parallel = central_parallel.to_radians();
        let central_meridian = central_meridian.to_radians();
        match *self {
            Coordinates::Geographic { longitude, latitude } => {
                let latitide = latitude.to_radians();
                let longitude = longitude.to_radians();
                let x = longitude - central_meridian;
                let x = EARTH_RADIUS * x;
                let x = standard_parallel.cos() * x;
                let y = latitide - central_parallel;
                let y = EARTH_RADIUS * y;
                (x, y)
            },
        }
    }
}

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

    pub fn intersects(&self, other: &Line) -> bool {
        let p1 = self.a().project_geographic(STANDARD_PARALLEL, CENTRAL_PARALLEL, CENTRAL_MERIDIAN);
        let p2 = self.b().project_geographic(STANDARD_PARALLEL, CENTRAL_PARALLEL, CENTRAL_MERIDIAN);
        let p3 = other.a().project_geographic(STANDARD_PARALLEL, CENTRAL_PARALLEL, CENTRAL_MERIDIAN);
        let p4 = other.b().project_geographic(STANDARD_PARALLEL, CENTRAL_PARALLEL, CENTRAL_MERIDIAN);

        fn ccw(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
            (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
        }

        /*
            If p3 and p4 are on opposite sides of p1p2, one result will be positive
            while one will be negative, resulting in a negative product.
            Likewise, if p3 and p4 are on the *same* side of p1p2, either both results
            will be positive, or both will be negative, yielding a positive product.
            Therefore, positive products means that p3 and p4 are on the same side of p1p2,
            while negative means they're on the opposite.

            In order to have an intersection, we need to have that the result of checking p3, p4 with p1p2
            is negative AND that checking p1, p2 with p3p4 is negative, as both are line segments, not lines.
        */

        let test_1 = ccw(p1, p2, p3) * ccw(p1, p2, p4) < 0.0;
        let test_2 = ccw(p3, p4, p1) * ccw(p3, p4, p2) < 0.0;
        test_1 && test_2
    }
}