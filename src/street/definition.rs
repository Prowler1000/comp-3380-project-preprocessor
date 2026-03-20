use std::{fmt::Display, num::ParseFloatError, sync::LazyLock};

use csv::Reader;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{LOCATION_REGEX, geospatial::{Coordinates, Line}};

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialStreetDefinition {
    pub name: String,
    pub qualifier: String,
    pub begin_measure: f64,
    pub end_measure: f64,
    pub location: Vec<Line>,
}

#[derive(Debug, Error)]
pub enum SDFromCSVError {
    #[error("The reader failed to get headers. {_0}")]
    ReadHeadersFailed(#[source] csv::Error),
    #[error("The header \"{}\" was expected but was missing", _0)]
    MissingHeader(&'static str),
    #[error("Multiple definitions of the header \"{_0}\" were found.")]
    RepeatHeader(&'static str),
    #[error("An unexpected header was found. {}", _0)]
    UnknownHeader(String),
}

#[derive(Debug, Error)]
pub struct SDEntryParseError {
    pub line: usize,
    pub error: StreetDefinitionParseError,
}

impl Display for SDEntryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {}: ", self.line)?;
        self.error.fmt(f)
    }
}

#[derive(Debug, Error)]
pub enum StreetDefinitionParseError {
    #[error("Failed to read the line. {:#?}", _0)]
    ReadFailure(#[source] csv::Error),
    #[error("The \"{}\" entry returned None", _0)]
    MissingEntry(&'static str),
    #[error("Failed to parse one or more coordinates. \"{}\" {}", value, source)]
    CoordinateParseError {
        value: String,
        source: ParseFloatError,
    },
    #[error("Failed to parse one of the measures. \"{}\" {}", value, source)]
    MeasureParseError {
        value: String,
        source: ParseFloatError,
    },
}

fn new_error(line: usize, error: impl Into<StreetDefinitionParseError>) -> SDEntryParseError {
    SDEntryParseError {
        line,
        error: error.into(),
    }
}
impl PartialStreetDefinition {
    pub fn from_csv<R>(
        mut reader: Reader<R>,
    ) -> Result<Vec<Result<Self, SDEntryParseError>>, SDFromCSVError>
    where
        R: std::io::Read,
    {
        let headers = reader
            .headers()
            .map_err(SDFromCSVError::ReadHeadersFailed)?;
        let mut street_name_index: Option<usize> = None;
        let mut street_qualifier_index: Option<usize> = None;
        let mut begin_measure_index: Option<usize> = None;
        let mut end_measure_index: Option<usize> = None;
        let mut location_index: Option<usize> = None;
        for (index, header) in headers.iter().map(str::to_ascii_lowercase).enumerate() {
            match header.as_str() {
                "street name" => {
                    if street_name_index.is_some() {
                        return Err(SDFromCSVError::RepeatHeader("street name"));
                    }
                    street_name_index = Some(index);
                }
                "street qualifier" => {
                    if street_qualifier_index.is_some() {
                        return Err(SDFromCSVError::RepeatHeader("street qualifier"));
                    }
                    street_qualifier_index = Some(index);
                }
                "begin measure" => {
                    if begin_measure_index.is_some() {
                        return Err(SDFromCSVError::RepeatHeader("begin measure"));
                    }
                    begin_measure_index = Some(index);
                }
                "end measure" => {
                    if end_measure_index.is_some() {
                        return Err(SDFromCSVError::RepeatHeader("end measure"));
                    }
                    end_measure_index = Some(index);
                }
                "location" => {
                    if location_index.is_some() {
                        return Err(SDFromCSVError::RepeatHeader("location"));
                    }
                    location_index = Some(index);
                }
                _ => return Err(SDFromCSVError::UnknownHeader(header)),
            }
        }
        let street_name_index =
            street_name_index.ok_or(SDFromCSVError::MissingHeader("street name"))?;
        let street_qualifier_index =
            street_qualifier_index.ok_or(SDFromCSVError::MissingHeader("street qualifier"))?;
        let begin_measure_index =
            begin_measure_index.ok_or(SDFromCSVError::MissingHeader("begin measure"))?;
        let end_measure_index =
            end_measure_index.ok_or(SDFromCSVError::MissingHeader("end measure"))?;
        let location_index = location_index.ok_or(SDFromCSVError::MissingHeader("location"))?;
        let streets = reader
            .into_records()
            .enumerate()
            .map(|(line, record_result)| -> Result<Self, SDEntryParseError> {
                let line = line + 2;
                let record = record_result.map_err(|error| {
                    new_error(line, StreetDefinitionParseError::ReadFailure(error))
                })?;
                let name = record.get(street_name_index).ok_or(new_error(
                    line,
                    StreetDefinitionParseError::MissingEntry("street name"),
                ))?;
                let qualifier = record.get(street_qualifier_index).ok_or(new_error(
                    line,
                    StreetDefinitionParseError::MissingEntry("street qualifier"),
                ))?;
                let begin_measure = record.get(begin_measure_index).ok_or(new_error(
                    line,
                    StreetDefinitionParseError::MissingEntry("begin measure"),
                ))?;
                let end_measure = record.get(end_measure_index).ok_or(new_error(
                    line,
                    StreetDefinitionParseError::MissingEntry("end measure"),
                ))?;
                let location = record.get(location_index).ok_or(new_error(
                    line,
                    StreetDefinitionParseError::MissingEntry("location"),
                ))?;
                Self::try_parse_record(name, qualifier, begin_measure, end_measure, location)
                    .map_err(|error| new_error(line, error))
            })
            .collect::<Vec<_>>();
        Ok(streets)
    }

    pub fn try_parse_record(
        name: &str,
        qualifier: &str,
        begin_measure: &str,
        end_measure: &str,
        location: &str,
    ) -> Result<Self, StreetDefinitionParseError> {
        let coordinates = LOCATION_REGEX
            .captures_iter(location)
            .map(
                |captures| -> Result<Coordinates, StreetDefinitionParseError> {
                    let latitude = &captures["latitude"];
                    let longitude = &captures["longitude"];
                    Ok(Coordinates::Geographic {
                        longitude: longitude.parse::<f64>().map_err(|error| {
                            StreetDefinitionParseError::CoordinateParseError {
                                value: longitude.to_owned(),
                                source: error,
                            }
                        })?,
                        latitude: latitude.parse::<f64>().map_err(|error| {
                            StreetDefinitionParseError::CoordinateParseError {
                                value: latitude.to_owned(),
                                source: error,
                            }
                        })?,
                    })
                },
            )
            .collect::<Result<Vec<_>, _>>()?;
        let location = coordinates
            .array_windows()
            .map(|[first, second]| Line([*first, *second]))
            .collect::<Vec<_>>();
        let begin_measure = begin_measure.replace(",", "");
        let end_measure = end_measure.replace(",", "");
        Ok(Self {
            name: name.to_owned(),
            qualifier: qualifier.to_owned(),
            begin_measure: begin_measure.parse::<f64>().map_err(|error| {
                StreetDefinitionParseError::MeasureParseError {
                    value: begin_measure.to_owned(),
                    source: error,
                }
            })?,
            end_measure: end_measure.parse::<f64>().map_err(|error| {
                StreetDefinitionParseError::MeasureParseError {
                    value: end_measure.to_owned(),
                    source: error,
                }
            })?,
            location,
        })
    }
}
