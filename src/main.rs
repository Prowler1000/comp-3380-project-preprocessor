use std::{fs::File, io::Write};

use crate::street::StreetDefinition;

pub mod street;
pub mod geospatial;

static STREETS_JSON: &str = ".out/street_definitions.json";
static STREETS_JSON_PRETTY: &str = ".out/street_definitions_pretty.json";

fn main() {
    let streets_reader = csv::Reader::from_path(".raw_data/City_of_Winnipeg_LRS_20260313.csv").expect("Failed to create reader from path");
    let streets = StreetDefinition::from_csv(streets_reader).expect("Failed to parse street definitions from CSV");
    let streets_json = serde_json::to_string(&streets).expect("Failed to write street definitions to string");
    File::create(STREETS_JSON).expect("Failed to create streets json file").write_all(streets_json.as_bytes()).expect("Failed to write street definitions to file");
    let streets_json_pretty = serde_json::to_string_pretty(&streets).expect("Failed to write pretty street definitions to string");
    File::create(STREETS_JSON_PRETTY).expect("Failed to create street json pretty file").write_all(streets_json_pretty.as_bytes()).expect("Failed to write pretty street definitions to file");


}
