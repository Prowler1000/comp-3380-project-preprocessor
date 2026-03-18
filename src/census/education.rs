use std::collections::HashMap;

use anyhow::Context;
use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::{get_int_rounding, population::Population};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostSecondary(pub HashMap<String, Population>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighestCertDiplomaDegree(pub HashMap<String, u64>);

pub fn get_education(
    sheet: &Range<Data>,
) -> anyhow::Result<(PostSecondary, HighestCertDiplomaDegree)> {
    assert_matches!(sheet.get_value((369, 0)), Some(Data::String(str)) if str.trim() == "EDUCATION");
    Ok((get_post_secondary(sheet)?, get_highest_cert_diploma_or_degree(sheet)?))
}

pub fn get_post_secondary(sheet: &Range<Data>) -> anyhow::Result<PostSecondary> {
    let mut map = HashMap::new();
    for row in sheet.range((371, 0), (382, 2)).rows() {
        match row[0].clone() {
            Data::String(education) => {
                map.insert(
                    education.trim().to_owned(),
                    Population {
                        male: get_int_rounding(Some(&row[1]))
                            .context("No or invalid entry for male count")?
                            as u64,
                        female: get_int_rounding(Some(&row[2]))
                            .context("No or invalid entry for female count")?
                            as u64,
                    },
                );
            }
            invalid_education => {
                return Err(anyhow::Error::msg(format!(
                    "Expected String, got {:#?}",
                    invalid_education
                )));
            }
        }
    }
    Ok(PostSecondary(map))
}

pub fn get_highest_cert_diploma_or_degree(sheet: &Range<Data>) -> anyhow::Result<HighestCertDiplomaDegree> {
    let mut map = HashMap::new();
    for row in sheet.range((387, 0), (395, 1)).rows() {
        match row[0].clone() {
            Data::String(thing) => {
                map.insert(thing.trim().to_owned(), get_int_rounding(Some(&row[1])).context("Invalid entry for count")? as u64);
            },
            invalid => {
                return Err(anyhow::Error::msg(format!(
                    "Expected String, got {:#?}",
                    invalid
                )));
            }
        }
    }
    Ok(HighestCertDiplomaDegree(map))
}