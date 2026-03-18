use std::{f64, ops::Add};

use anyhow::Context;
use calamine::{Data, Range};
use comp_3380_project_preprocessor::Datapoint;
use serde::{Deserialize, Serialize};

use crate::census::{get_float, get_float_datapoint, get_int_rounding};

pub mod by_age;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Population<T = u64> {
    pub male: T,
    pub female: T,
}

impl Add for Population {
    type Output = Population;

    fn add(self, rhs: Self) -> Self::Output {
        Population {
            male: self.male + rhs.male,
            female: self.female + rhs.female,
        }
    }
}

impl<T> From<Population<T>> for Population<Datapoint<T>> {
    fn from(value: Population<T>) -> Self {
        Population { male: Datapoint::Datapoint(value.male), female: Datapoint::Datapoint(value.female) }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TotalPopulation {
    pub census_2021: u64,
    pub census_2016: u64,
    pub census_2011: u64,
    pub census_2006: u64,
    pub census_2001: u64,
    pub census_1996: u64,
    pub census_1991: u64,
}

pub fn get_population(sheet: &Range<Data>) -> anyhow::Result<TotalPopulation> {
    let sub_range = sheet.range((52, 0), (60, 2));
    assert_eq!(
        sub_range.get((0, 0)),
        Some(&Data::String("TOTAL POPULATION".to_owned()))
    );
    assert_eq!(
        sub_range.get((8, 0)),
        Some(&Data::String("1991 CENSUS".to_owned()))
    );
    Ok(TotalPopulation {
        census_2021: get_int_rounding(sub_range.get((2, 1)))
            .context("No or invalid 2021 population")? as u64,
        census_2016: get_int_rounding(sub_range.get((3, 1)))
            .context("No or invalid 2016 population")? as u64,
        census_2011: get_int_rounding(sub_range.get((4, 1)))
            .context("No or invalid 2011 population")? as u64,
        census_2006: get_int_rounding(sub_range.get((5, 1)))
            .context("No or invalid 2006 population")? as u64,
        census_2001: get_int_rounding(sub_range.get((6, 1)))
            .context("No or invalid 2001 population")? as u64,
        census_1996: get_int_rounding(sub_range.get((7, 1)))
            .context("No or invalid 1996 population")? as u64,
        census_1991: get_int_rounding(sub_range.get((8, 1)))
            .context("No or invalid 1991 population")? as u64,
    })
}

pub fn get_row_int_population(sheet: &Range<Data>, row: u32) -> anyhow::Result<Population> {
    Ok(Population {
        male: get_int_rounding(sheet.get_value((row, 1)))
            .context("No or invalid male count value")? as u64,
        female: get_int_rounding(sheet.get_value((row, 2)))
            .context("No or invalid female count value")? as u64,
    })
}

pub fn get_row_float_population(sheet: &Range<Data>, row: u32) -> anyhow::Result<Population<Datapoint<f64>>> {
    Ok(Population {
        male: get_float_datapoint(sheet.get_value((row, 1))).context("No or invalid male float value")?,
        female: get_float_datapoint(sheet.get_value((row, 2))).context("No or invalid female float value")?,
    })
}
