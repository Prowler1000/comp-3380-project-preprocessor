use serde::{Deserialize, Serialize};

use crate::geospatial::{Coordinates, Line};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Polygon(Vec<Coordinates>);

#[derive(Debug, Default, PartialEq)]
pub struct PolygonBuilder {
    points: Vec<Coordinates>,
}

impl Polygon {
    pub fn create_lines(&self) -> Vec<Line> {
        self.0
            .array_windows()
            .copied()
            .map(Line)
            .collect::<Vec<_>>()
    }

    pub fn points(&self) -> &[Coordinates] {
        &self.0
    }
}

impl PolygonBuilder {
    /// Add a point to the polygon.
    ///
    /// # Error
    /// If the point would create a line that intersects another line,
    /// return the line it would intersect in an Err
    pub fn add_point(&mut self, point: Coordinates) -> Result<(), Line> {
        if let Some(last) = self.points.last().copied() {
            let potential_line = Line::new(last, point);
            if let Some(conflict_line) = self
                .points
                .array_windows()
                .copied()
                .map(Line)
                .find(|line| *line != potential_line && potential_line.intersects(line))
            {
                return Err(conflict_line);
            }
        }
        self.points.push(point);
        Ok(())
    }

    pub fn with_point(mut self, point: Coordinates) -> Result<Self, (Line, Self)> {
        if let Err(line) = self.add_point(point) {
            Err((line, self))
        } else {
            Ok(self)
        }
    }

    /// Try to build the polygon, verifying that the polygon is valid,
    /// returning self if not.
    ///
    /// # Validity
    /// A polygon is valid if it has 3 or more verticies (4 or more points)
    /// and the first and last points equal
    pub fn try_build(self) -> Result<Polygon, Self> {
        if self.points.len() > 3
            && self
                .points
                .first()
                .zip(self.points.last())
                .is_some_and(|(first, last)| first.eq(last))
        {
            Ok(Polygon(self.points))
        } else {
            Err(self)
        }
    }
}
