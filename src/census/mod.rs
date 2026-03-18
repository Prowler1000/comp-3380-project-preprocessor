use std::path::Path;

use anyhow::Context;
use assert_matches::assert_matches;
use calamine::{Data, Range, Reader, open_workbook_auto};
use serde::{Deserialize, Serialize};

use crate::census::{
    citizenship::{Citizenship, get_citizenship}, education::{Education, get_education}, immigration::{Immigration, get_immigration}, income::{Income, get_income}, indigenous::{Indigenous, get_indigenous}, labour::{Labour, get_labour}, language::{Language, get_languages}, marital_status::{MaritalStatus, get_marital_status}, population::{
        TotalPopulation,
        by_age::{PopulationByAge, get_population_by_age},
        get_population,
    }, religion::{Religion, get_religion}, transportation::{MainTransportation, get_transportation}, visible_minorities::{VisibleMinorities, get_visible_minorities}
};

pub mod citizenship;
pub mod education;
pub mod immigration;
pub mod indigenous;
pub mod labour;
pub mod language;
pub mod marital_status;
pub mod population;
pub mod religion;
pub mod transportation;
pub mod visible_minorities;
pub mod income;

pub fn assert_get_counts(
    sheet: &Range<Data>,
    tests: impl Iterator<Item = (u32, impl AsRef<str>)>,
) -> impl Iterator<Item = anyhow::Result<u64>> {
    tests.map(|(row, test_value)| {
        assert_matches!(sheet.get_value((row, 0)), Some(Data::String(str)) if str.trim() == test_value.as_ref(), "Test value: {}", test_value.as_ref());
        Ok(get_int_rounding(sheet.get_value((row, 1))).context(format!("Invalid entry for {}", test_value.as_ref()))? as u64)
    })
}

pub fn assert_test_ranges(
    sheet: &Range<Data>,
    mut tests: impl Iterator<Item = ((u32, u32), impl Into<String>)>,
) {
    tests.all(|(range, value)| {
        assert_matches!(sheet.get_value(range), Some(Data::String(str)) if str.trim() == value.into().as_str());
        true
    });
}

/// Get a number and round it if it's a float
pub(crate) fn get_int_rounding(data: Option<&Data>) -> Option<i64> {
    match data {
        Some(Data::Int(val)) => Some(*val),
        Some(Data::Float(val)) => Some(val.round() as i64),
        Some(_) | None => None,
    }
}

pub(crate) fn get_float<F>(data: Option<&Data>, on_not_applicable: F) -> anyhow::Result<f64> where F: FnOnce() -> anyhow::Result<f64> {
    match data {
        Some(Data::Float(val)) => Ok(*val),
        Some(Data::String(str)) if str.trim() == "..." => {
            on_not_applicable()
        },
        invalid => Err(anyhow::Error::msg(format!("Expected a float, found {:#?}", invalid))),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CensusData {
    total_population: TotalPopulation,
    population_by_age: PopulationByAge,
    languages: Language,
    indigenous: Indigenous,
    visible_minorities: VisibleMinorities,
    citizenship: Citizenship,
    immigration: Immigration,
    religion: Religion,
    marital_status: MaritalStatus,
    education: Education,
    labour: Labour,
    transportation: MainTransportation,
    income: Income,
}

pub fn get_census_data<P>(path: P) -> anyhow::Result<CensusData>
where
    P: AsRef<Path>,
{
    let mut workbook = open_workbook_auto(path).context("Failed to open workbook")?;
    let sheet = workbook
        .worksheet_range_at(0)
        .context("Failed to get data worksheet")??;
    let total_population = get_population(&sheet)?;
    let population_by_age = get_population_by_age(&sheet)?;
    let visible_minorities = get_visible_minorities(&sheet)?;
    let citizenship = get_citizenship(&sheet)?;
    let religion = get_religion(&sheet)?;
    let marital_status = get_marital_status(&sheet)?;
    let transportation = get_transportation(&sheet)?;
    Ok(CensusData {
        total_population,
        population_by_age,
        languages: get_languages(&sheet)?,
        indigenous: get_indigenous(&sheet)?,
        visible_minorities,
        citizenship,
        immigration: get_immigration(&sheet)?,
        religion,
        marital_status,
        education: get_education(&sheet)?,
        labour: get_labour(&sheet)?,
        transportation,
        income: get_income(&sheet)?,
    })
}

#[allow(unused)]
pub fn test() -> anyhow::Result<()> {
    static TEST_PATH: &str = ".raw_data/Amber Trails.xlsx";
    let mut workbook = open_workbook_auto(TEST_PATH).context("Failed to open workbook")?;
    let sheet = workbook
        .worksheet_range("Amber Trails Profile")
        .context("Idk, failed to get worksheet range")?;
    let population = get_population(&sheet)?;
    let pop_by_age = get_population_by_age(&sheet)?;
    let languages = get_languages(&sheet)?;
    let indigenous = get_indigenous(&sheet)?;
    let visible_minorities = get_visible_minorities(&sheet)?;
    let citizenship = get_citizenship(&sheet)?;
    let immigration = get_immigration(&sheet)?;
    let religion = get_religion(&sheet)?;
    let marital_status = get_marital_status(&sheet)?;
    let education = get_education(&sheet)?;
    let labour = get_labour(&sheet)?;
    let transportation = get_transportation(&sheet)?;
    println!("{:#?}", population);
    println!("{:#?}", pop_by_age);
    println!("{:#?}", languages);
    println!("{:#?}", indigenous);
    println!("{:#?}", visible_minorities);
    println!("{:#?}", citizenship);
    println!("{:#?}", immigration);
    Ok(())
}
