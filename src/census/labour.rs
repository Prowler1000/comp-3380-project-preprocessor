use std::collections::HashMap;

use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::{
    get_float, get_int_rounding,
    population::{Population, get_row_int_population},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Labour {
    labour_force_activity: LabourForceActivity,
    employment_sector: EmploymentSector,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LabourForceActivity {
    employed: Population,
    unemployed: Population,
    not_in_labour_force: Population,
    participation_rate: Population<f64>,
    employment_rate: Population<f64>,
    unemployment_rate: Population<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmploymentSector(pub HashMap<String, u64>);

pub fn get_labour(sheet: &Range<Data>) -> anyhow::Result<Labour> {
    assert_matches!(sheet.get_value((402, 0)), Some(Data::String(str)) if str.trim() == "LABOUR FORCE ACTIVITY");
    Ok(Labour {
        labour_force_activity: get_labour_force_activity(sheet)?,
        employment_sector: get_employment_sector(sheet)?,
    })
}

fn get_labour_force_activity(sheet: &Range<Data>) -> anyhow::Result<LabourForceActivity> {
    Ok(LabourForceActivity {
        employed: get_row_int_population(sheet, 405)?,
        unemployed: get_row_int_population(sheet, 406)?,
        not_in_labour_force: get_row_int_population(sheet, 407)?,
        participation_rate: Population {
            male: get_float(sheet.get_value((408, 1)), || Ok(f64::NAN)).unwrap(),
            female: get_float(sheet.get_value((408, 2)), || Ok(f64::NAN)).unwrap(),
        },
        employment_rate: Population {
            male: get_float(sheet.get_value((409, 1)), || Ok(f64::NAN)).unwrap(),
            female: get_float(sheet.get_value((409, 2)), || Ok(f64::NAN)).unwrap(),
        },
        unemployment_rate: Population {
            male: get_float(sheet.get_value((410, 1)), || Ok(f64::NAN)).unwrap(),
            female: get_float(sheet.get_value((410, 2)), || Ok(f64::NAN)).unwrap(),
        },
    })
}

fn get_employment_sector(sheet: &Range<Data>) -> anyhow::Result<EmploymentSector> {
    let mut map = HashMap::<String, u64>::new();
    for row in sheet.range((442, 0), (461, 1)).rows() {
        match row[0].clone() {
            Data::String(sector) => {
                map.insert(
                    sector.trim().to_owned(),
                    get_int_rounding(Some(&row[1])).unwrap() as u64,
                );
            }
            _ => panic!("Idk, wrong data type for employement sector"),
        }
    }
    Ok(EmploymentSector(map))
}
