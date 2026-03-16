use std::collections::HashMap;

use anyhow::Context;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::{geospatial::Line, street::definition::PartialStreetDefinition};

pub mod definition;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StreetDefinition {
    pub name: String,
    pub qualifiers: HashMap<String, StreetQualifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StreetQualifier {
    pub measures: Vec<Measure>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Measure {
    pub begin: f64,
    pub end: f64,
    pub lines: Vec<Line>,
}

impl StreetDefinition {
    pub fn from_csv<R>(reader: Reader<R>) -> anyhow::Result<Vec<Self>>
    where
        R: std::io::Read,
    {
        let streets = PartialStreetDefinition::from_csv(reader)
            .context("Failed to parse streets from CSV")?;
        // Effectively transpose the first error, if any, and the container
        let streets = streets.into_iter().collect::<Result<Vec<_>, _>>().context("\
            One or more lines of the CSV failed to parse. \
            Given that street definitions are split between multiple lines, this has been treated as unrecoverable\
        ")?;
        let mut map = HashMap::<String, StreetDefinition>::new();
        for street in streets {
            let street_entry = map
                .entry(street.name.clone())
                .or_insert_with(|| StreetDefinition {
                    name: street.name.clone(),
                    qualifiers: HashMap::new(),
                });
            let qualifier = street_entry
                .qualifiers
                .entry(street.qualifier.clone())
                .or_insert_with(|| StreetQualifier {
                    measures: Vec::new(),
                });
            qualifier.measures.push(Measure {
                begin: street.begin_measure,
                end: street.end_measure,
                lines: street.location,
            });
        }
        Ok(map.into_values().collect::<Vec<_>>())
    }
}
