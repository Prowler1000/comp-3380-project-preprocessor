use std::collections::HashMap;

use anyhow::Context;
use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::{assert_get_counts, get_int_rounding};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Immigration {
    pub birthplace: Birthplace,
    pub period: ImmigrationPeriod,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Birthplace(pub HashMap<String, u64>);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ImmigrationPeriod {
    before_1980: u64,
    from_1980_to_1990: u64,
    from_1991_to_2000: u64,
    from_2000_to_2010: u64,
    from_2011_to_2015: u64,
    from_2016_to_2021: u64,
}

pub fn get_immigration(sheet: &Range<Data>) -> anyhow::Result<Immigration> {
    Ok(Immigration {
        birthplace: get_birthplace(sheet)?,
        period: get_immigration_period(sheet)?
    })
}

fn get_immigration_period(sheet: &Range<Data>) -> anyhow::Result<ImmigrationPeriod> {
    let mut items = assert_get_counts(sheet, 
    [
        (269, "Before 1980"),
        (270, "1980 to 1990"),
        (271, "1991 to 2000"),
        (272, "2001 to 2010"),
        (274, "2011 to 2015"),
        (275, "2016 to 2021"),
    ].into_iter());
    Ok(
        ImmigrationPeriod {
            before_1980: items.next().unwrap()?,
            from_1980_to_1990: items.next().unwrap()?,
            from_1991_to_2000: items.next().unwrap()?,
            from_2000_to_2010: items.next().unwrap()?,
            from_2011_to_2015: items.next().unwrap()?,
            from_2016_to_2021: items.next().unwrap()?,
        }
    )
}

fn get_birthplace(sheet: &Range<Data>) -> anyhow::Result<Birthplace> {
    assert_matches!(sheet.get_value((230, 0)), Some(Data::String(str)) if str.trim() == "IMMIGRATION");
    assert_matches!(sheet.get_value((231, 0)), Some(Data::String(str)) if str.trim() == "Place of Birth");
    let mut map = HashMap::<String, u64>::new();
    let sub_range = sheet.range((232, 0), (261, 1));
    for row in sub_range.rows() {
        match (row[0].clone(), row[1].clone()) {
            (Data::String(birthplace), Data::Int(count)) => {
                map.insert(birthplace.trim().to_owned(), count as u64);
            },
            (Data::String(birthplace), Data::Float(count)) => {
                assert!(count.is_sign_positive());
                assert_eq!(count.fract(), 0.0);
                map.insert(birthplace.trim().to_owned(), count as u64);
            },
            (invalid_birthplace, invalid_count) => {
                return Err(anyhow::Error::msg(format!("Expected String and Int/Float, got {:#?} and {:#?}", invalid_birthplace, invalid_count)));
            }
        }
    }
    map.insert("Other".to_owned(), get_int_rounding(sheet.get_value((262, 1))).context("No or invalid count for other")? as u64);
    Ok(Birthplace(map))
}