use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::{
    census::{CensusData, get_census_data, test},
    neighbourhood::boundary::NeighbourhoodBoundary,
    street::StreetDefinition,
};

pub mod census;
pub mod geospatial;
pub mod neighbourhood;
pub mod street;

static RAW_STREETS: &str = ".raw_data/City_of_Winnipeg_LRS_20260313.csv";
static RAW_NEIGHBOURHOODS: &str = ".raw_data/Neighbourhoods_20260313.csv";

static STREETS_JSON: &str = ".out/street_definitions.json";
static STREETS_JSON_PRETTY: &str = ".out/street_definitions_pretty.json";

static NEIGHBOURHOODS_JSON: &str = ".out/neighbourhood_boundaries.json";

#[allow(unused)]
fn parse_streets() {
    let streets_reader =
        csv::Reader::from_path(RAW_STREETS).expect("Failed to create streets reader from path");
    let streets = StreetDefinition::from_csv(streets_reader)
        .expect("Failed to parse street definitions from CSV");
    let streets_json =
        serde_json::to_string(&streets).expect("Failed to write street definitions to string");
    File::create(STREETS_JSON)
        .expect("Failed to create streets json file")
        .write_all(streets_json.as_bytes())
        .expect("Failed to write street definitions to file");
    let streets_json_pretty = serde_json::to_string_pretty(&streets)
        .expect("Failed to write pretty street definitions to string");
    File::create(STREETS_JSON_PRETTY)
        .expect("Failed to create street json pretty file")
        .write_all(streets_json_pretty.as_bytes())
        .expect("Failed to write pretty street definitions to file");
}

#[allow(unused)]
fn parse_neighbourhoods() {
    let reader = csv::Reader::from_path(RAW_NEIGHBOURHOODS)
        .expect("Failed to create neighbourhoods reader from path");
    let neighbourhoods =
        NeighbourhoodBoundary::from_csv(reader).expect("Failed to parse neighbourhoods csv data");
    let neighbourhoods_json = serde_json::to_string_pretty(&neighbourhoods)
        .expect("Failed to write neighbourhoods to json string");
    File::create(NEIGHBOURHOODS_JSON)
        .expect("Failed to create neighbourhoods json file")
        .write_all(neighbourhoods_json.as_bytes())
        .expect("Failed to write neighbourhoods json to file");
}

fn save_census_data<P>(data: CensusData, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let data_json = serde_json::to_string_pretty(&data)
        .context("Failed to write census data to json string")?;
    File::create(path.as_ref())
        .context(format!(
            "Failed to create json file \"{}\"",
            path.as_ref().display()
        ))?
        .write_all(data_json.as_bytes())
        .context("Failed to write census data to file")?;
    Ok(())
}

fn main() {
    // parse_streets();
    // parse_neighbourhoods();
    const IN_DIR: &str = ".raw_data";
    const OUT_DIR: &str = ".out";
    for item in std::fs::read_dir(IN_DIR).expect("Failed to read IN_DIR") {
        let entry = item.expect("Failed to walk IN_DIR directory");
        if entry
            .path()
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("xlsx"))
            && entry
                .path()
                .file_prefix()
                .and_then(|pfx| pfx.to_str())
                .is_some_and(|pfx| !pfx.starts_with("~"))
        {
            let data = get_census_data(entry.path())
                .unwrap_or_else(|e| panic!("Failed to parse {}. {:#?}", entry.path().display(), e));
            let mut new_path = PathBuf::new()
                .join(OUT_DIR)
                .join(entry.path().file_prefix().unwrap());
            new_path.set_extension("json");
            save_census_data(data, new_path).expect("Failed to save the data to file");
        }
    }
}
