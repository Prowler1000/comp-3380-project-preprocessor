use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::{Datapoint, census::{assert_test_ranges, population::{Population, get_row_float_population}}};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Income {
    composition: PercentCompositionOfTotalIncome,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PercentCompositionOfTotalIncome {
    market: MarketIncome,
    government_transfers: GovernmentTransfers,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MarketIncome {
    total: Population<Datapoint<f64>>,
    employment: Population<Datapoint<f64>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GovernmentTransfers {
    total: Population<Datapoint<f64>>,
    employment_insurance: Population<Datapoint<f64>>,
    covid_19_income_support: Population<Datapoint<f64>>,
    covid_19_emergency: Population<Datapoint<f64>>,
}

pub struct IndividualIncome {}

pub fn get_income(sheet: &Range<Data>) -> anyhow::Result<Income> {
    assert_matches!(sheet.get_value((486, 0)), Some(Data::String(str)) if str.trim() == "INCOME");
    Ok(Income {
        composition: get_percent_composition(sheet)?,
    })
}

fn get_percent_composition(sheet: &Range<Data>) -> anyhow::Result<PercentCompositionOfTotalIncome> {
    assert_test_ranges(sheet, 
    [
        ((488, 0), "Market income (%)"),
        ((489, 0), "Employment income (%)"),
        ((490, 0), "Government transfers (%)"),
        ((491, 0), "Employment insurance benefits (%)"),
        ((492, 0), "COVID-19 - Government income support and benefits (%)"),
        ((493, 0), "COVID-19 - Emergency and recovery benefits (%)"),
    ].into_iter());
    Ok(PercentCompositionOfTotalIncome {
        market: MarketIncome {
            total: get_row_float_population(sheet, 488)?,
            employment: get_row_float_population(sheet, 489)?,
        },
        government_transfers: GovernmentTransfers {
            total: get_row_float_population(sheet, 490)?,
            employment_insurance: get_row_float_population(sheet, 491)?,
            covid_19_income_support: get_row_float_population(sheet, 492)?,
            covid_19_emergency: get_row_float_population(sheet, 493)?,
        },
    })
}
