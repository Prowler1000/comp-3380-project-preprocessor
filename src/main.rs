use std::{fs::File, io::Write};

use crate::{census::test, neighbourhood::boundary::NeighbourhoodBoundary, street::StreetDefinition};

pub mod street;
pub mod neighbourhood;
pub mod geospatial;
pub mod census;

static RAW_STREETS: &str = ".raw_data/City_of_Winnipeg_LRS_20260313.csv";
static RAW_NEIGHBOURHOODS: &str = ".raw_data/Neighbourhoods_20260313.csv";

static STREETS_JSON: &str = ".out/street_definitions.json";
static STREETS_JSON_PRETTY: &str = ".out/street_definitions_pretty.json";

static NEIGHBOURHOODS_JSON: &str = ".out/neighbourhood_boundaries.json";

#[allow(unused)]
fn parse_streets() {
    let streets_reader = csv::Reader::from_path(RAW_STREETS).expect("Failed to create streets reader from path");
    let streets = StreetDefinition::from_csv(streets_reader).expect("Failed to parse street definitions from CSV");
    let streets_json = serde_json::to_string(&streets).expect("Failed to write street definitions to string");
    File::create(STREETS_JSON).expect("Failed to create streets json file").write_all(streets_json.as_bytes()).expect("Failed to write street definitions to file");
    let streets_json_pretty = serde_json::to_string_pretty(&streets).expect("Failed to write pretty street definitions to string");
    File::create(STREETS_JSON_PRETTY).expect("Failed to create street json pretty file").write_all(streets_json_pretty.as_bytes()).expect("Failed to write pretty street definitions to file");
}

#[allow(unused)]
fn parse_neighbourhoods() {
    let reader = csv::Reader::from_path(RAW_NEIGHBOURHOODS).expect("Failed to create neighbourhoods reader from path");
    let neighbourhoods = NeighbourhoodBoundary::from_csv(reader).expect("Failed to parse neighbourhoods csv data");
    let neighbourhoods_json = serde_json::to_string_pretty(&neighbourhoods).expect("Failed to write neighbourhoods to json string");
    File::create(NEIGHBOURHOODS_JSON).expect("Failed to create neighbourhoods json file").write_all(neighbourhoods_json.as_bytes()).expect("Failed to write neighbourhoods json to file");
}

fn main() {
    // parse_streets();
    // parse_neighbourhoods();
    test().expect("Failed to do the thing");
}
