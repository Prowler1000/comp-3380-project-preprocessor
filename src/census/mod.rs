use anyhow::Context;
use assert_matches::assert_matches;
use calamine::{Data, Range, Reader, open_workbook_auto};

use crate::census::{citizenship::get_citizenship, education::get_education, immigration::get_immigration, indigenous::get_indigenous, labour::get_labour, language::get_languages, marital_status::get_marital_status, population::{by_age::get_population_by_age, get_population}, religion::get_religion, transportation::get_transportation, visible_minorities::get_visible_minorities};

pub mod population;
pub mod language;
pub mod indigenous;
pub mod visible_minorities;
pub mod citizenship;
pub mod immigration;
pub mod religion;
pub mod marital_status;
pub mod education;
pub mod labour;
pub mod transportation;

pub fn assert_get_counts(sheet: &Range<Data>, tests: impl Iterator<Item = (u32, impl AsRef<str>)>) -> impl Iterator<Item = anyhow::Result<u64>> {
    tests.map(|(row, test_value)| {
        assert_matches!(sheet.get_value((row, 0)), Some(Data::String(str)) if str.trim() == test_value.as_ref(), "Test value: {}", test_value.as_ref());
        Ok(get_int_rounding(sheet.get_value((row, 1))).context(format!("Invalid entry for {}", test_value.as_ref()))? as u64)
    })
}

pub fn assert_test_ranges(sheet: &Range<Data>, mut tests: impl Iterator<Item = ((u32, u32), impl Into<String>)>) {
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

pub(crate) fn get_float(data: Option<&Data>) -> Option<f64> {
    match data {
        Some(Data::Float(val)) => Some(*val),
        _ => None,
    }
}

pub fn test() -> anyhow::Result<()> {
    static TEST_PATH: &str = ".raw_data/Amber Trails.xlsx";
    let mut workbook = open_workbook_auto(TEST_PATH).context("Failed to open workbook")?;
    let sheet = workbook.worksheet_range("Amber Trails Profile").context("Idk, failed to get worksheet range")?;
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
    // println!("{:#?}", population);
    // println!("{:#?}", pop_by_age);
    // println!("{:#?}", languages);
    // println!("{:#?}", indigenous);
    // println!("{:#?}", visible_minorities);
    // println!("{:#?}", citizenship);
    println!("{:#?}", immigration);
    Ok(())
}
