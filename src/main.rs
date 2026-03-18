use std::{
    collections::HashMap, ffi::OsString, fs::File, io::Write, path::{Path, PathBuf}
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

fn ensure_dir_exists<P>(path: P) -> std::io::Result<()> where P: AsRef<Path> {
    let path = path.as_ref();
    if !path.exists() {
        if path.extension().is_some() && let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        } else {
            std::fs::create_dir_all(path)?;
        }
    }
    Ok(())
}

#[allow(unused)]
fn parse_streets<P1, P2>(input_path: P1, output_path: P2) -> anyhow::Result<()> where P1: AsRef<Path>, P2: AsRef<Path> {
    let reader = csv::Reader::from_path(input_path).context("Failed to create street reader")?;
    let streets = StreetDefinition::from_csv(reader).context("Failed to parse street definitions from CSV")?;
    let streets_json = serde_json::to_string_pretty(&streets).context("Failed to parse streets to json string")?;
    ensure_dir_exists(output_path.as_ref()).context("Failed to create output directory")?;
    File::create(output_path).context("Failed to create output file")?.write_all(streets_json.as_bytes()).context("Failed to write streets json to file")
}

#[allow(unused)]
fn parse_neighbourhoods<P1, P2>(input_path: P1, output_path: P2) -> anyhow::Result<()> where P1: AsRef<Path>, P2: AsRef<Path> {
    let reader = csv::Reader::from_path(input_path).context("Failed to create neighbourhoods reader from path")?;
    let neighbourhoods = NeighbourhoodBoundary::from_csv(reader).context("Failed to parse neighbourhoods from CSV")?;
    let neighbourhoods_json = serde_json::to_string_pretty(&neighbourhoods).context("Failed to parse neighbourhoods to string")?;
    ensure_dir_exists(output_path.as_ref()).context("Failed to create output directory")?;
    File::create(output_path).context("Failed to create output file")?.write_all(neighbourhoods_json.as_bytes()).context("Failed to write neighbourhoods json to file")
}

fn save_census_data<P>(data: &CensusData, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let data_json = serde_json::to_string_pretty(data)
        .context("Failed to write census data to json string")?;
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent).context("Failed to create output directory")?;
    }
    File::create(path.as_ref())
        .context(format!(
            "Failed to create json file \"{}\"",
            path.as_ref().display()
        ))?
        .write_all(data_json.as_bytes())
        .context("Failed to write census data to file")?;
    Ok(())
}

fn save_aggregate_census_data<P>(data: impl Iterator<Item = (String, CensusData)>, path: P) -> anyhow::Result<()> where P: AsRef<Path> {
    let hash_aggregate = data.collect::<HashMap<_, _>>();
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent).context("Failed to create aggregate output directory")?;
    }
    let mut file = File::create(path.as_ref()).context(format!("Failed to create json file for aggregate census data at {}", path.as_ref().display()))?;
    let aggregate_string = serde_json::to_string_pretty(&hash_aggregate).context("Failed to serialize aggregate census data to string")?;
    file.write_all(aggregate_string.as_bytes()).context("Failed to write json string to file.")?;
    Ok(())
}

struct ExcelFile {
    file_name: String,
    excel_file_path: PathBuf,
}

fn main() {
    const IN_DIR: &str = ".raw_data";
    const OUT_DIR: &str = ".out";
    const AGGREGATE_OUT_DIR: &str = ".aggregate";

    // parse_streets(".raw_data/City_of_Winnipeg_LRS_20260313.csv", ".out/street_definitions.json").expect("Failed to parse streets");
    // parse_neighbourhoods(".raw_data/Neighbourhoods_20260313.csv", ".out/neighbourhood_boundaries.json").expect("Failed to parse neighbourhoods");
    let census_files = std::fs::read_dir(IN_DIR)
        .expect("Failed to read IN_DIR")
        .filter_map(|entry| -> Option<Result<ExcelFile, _>> {
            if let Err(error) = entry {
                return Some(Err(error));
            }
            let entry = entry.unwrap();
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(error) => {
                    return Some(Err(error));
                }
            };
            if file_type.is_file()
                && let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str())
                && ext == "xlsx"
                && let Some(name) = entry.path().file_name().and_then(|name| name.to_str()).and_then(|name| name.strip_suffix(".xlsx"))
                && !name.starts_with("~")
            {
                Some(Ok(ExcelFile{ file_name: name.to_owned(), excel_file_path: entry.path()}))
            } else {
                None
            }
        });
    let census_data = census_files.map(Result::unwrap).map(|ExcelFile { file_name, excel_file_path: path }| {
        let data = get_census_data(&path).unwrap_or_else(|e| panic!("Failed to get census data from {}. {:#?}", path.display(), e));
        let new_path = PathBuf::from(OUT_DIR).join(&file_name).with_extension("json");
        if let Err(error) = save_census_data(&data, new_path) {
            panic!("Failed to save census data for {} to file. {:#?}", path.display(), error);
        }
        (file_name, data)
    });
    save_aggregate_census_data(census_data, PathBuf::from(AGGREGATE_OUT_DIR).join("census.json")).expect("Failed to aggregate census data");
}
