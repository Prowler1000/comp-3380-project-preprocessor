use std::num::{ParseFloatError, ParseIntError};

use csv::Reader;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{LOCATION_REGEX, geospatial::{
    Coordinates,
    polygon::{Polygon, PolygonBuilder},
}};

#[derive(Debug, Serialize, Deserialize)]
pub struct NeighbourhoodBoundary {
    pub id: u64,
    pub name: String,
    pub polygon: Polygon,
}

#[derive(Debug, Error)]
pub enum NeighbourhoodBoundaryError {
    #[error("Failed to parse the headers.")]
    InvalidHeaders(csv::Error),
    #[error("The reader failed while reading the record on line {line}. {error}")]
    ReadRecordFailure { line: usize, error: csv::Error },
    #[error("Failed to parse the ID of the record on line {line} (\"{id}\") into an int. {error}")]
    IDParseFailure {
        line: usize,
        id: String,
        error: ParseIntError,
    },
    #[error("Failed to parse \"{value}\" into a coordinate component on line {line}. {error}")]
    CoordinateParseFailure {
        line: usize,
        value: String,
        error: ParseFloatError,
    },
    #[error("There is an intersection in the polygon on line {line}")]
    LineIntersection { line: usize },
    #[error("The polygon on line {line} is invalid")]
    InvalidPolygon { line: usize },
}

impl NeighbourhoodBoundary {
    pub fn from_csv<R>(mut reader: Reader<R>) -> Result<Vec<Self>, NeighbourhoodBoundaryError>
    where
        R: std::io::Read,
    {
        let headers = reader
            .headers()
            .map_err(NeighbourhoodBoundaryError::InvalidHeaders)?;
        // I can't be assed to be as thorough as I was with street definition.
        // We'll just panic if something isn't as expected
        assert_eq!(headers.get(0), Some("ID"));
        assert_eq!(headers.get(1), Some("Name"));
        assert_eq!(headers.get(2), Some("Location"));
        reader
            .into_records()
            .enumerate()
            .map(
                |(index, record_result)| -> Result<Self, NeighbourhoodBoundaryError> {
                    let line = index + 2;
                    let record = record_result.map_err(|error| {
                        NeighbourhoodBoundaryError::ReadRecordFailure { line, error }
                    })?;
                    let id_str = record
                        .get(0)
                        .unwrap_or_else(|| panic!("No ID for the record on line {}", line));
                    let name_str = record
                        .get(1)
                        .unwrap_or_else(|| panic!("No name for the record on line {}", line));
                    let location_str = record
                        .get(2)
                        .unwrap_or_else(|| panic!("No location for the record on line {}", line));
                    let id = id_str.parse::<u64>().map_err(|error| {
                        NeighbourhoodBoundaryError::IDParseFailure {
                            line,
                            id: id_str.into(),
                            error,
                        }
                    })?;
                    let mut points = LOCATION_REGEX.captures_iter(location_str).map(
                        |captures| -> Result<Coordinates, NeighbourhoodBoundaryError> {
                            let latitude_str = &captures["latitude"];
                            let longitude_str = &captures["longitude"];
                            let latitude = latitude_str.parse::<f64>().map_err(|error| {
                                NeighbourhoodBoundaryError::CoordinateParseFailure {
                                    line,
                                    value: latitude_str.into(),
                                    error,
                                }
                            })?;
                            let longitude = longitude_str.parse::<f64>().map_err(|error| {
                                NeighbourhoodBoundaryError::CoordinateParseFailure {
                                    line,
                                    value: longitude_str.into(),
                                    error,
                                }
                            })?;
                            Ok(Coordinates::Geographic {
                                longitude,
                                latitude,
                            })
                        },
                    );
                    let polygon_builder = points.try_fold(
                        PolygonBuilder::default(),
                        |builder, coordinates| -> Result<PolygonBuilder, NeighbourhoodBoundaryError> {
                            builder.with_point(coordinates?).map_err(|_| NeighbourhoodBoundaryError::LineIntersection { line })
                        },
                    )?;
                    let polygon = polygon_builder.try_build().map_err(|_| NeighbourhoodBoundaryError::InvalidPolygon { line })?;
                    Ok(Self { id, name: name_str.to_owned(), polygon })
                },
            )
            .collect::<Result<Vec<_>, _>>()
    }
}
