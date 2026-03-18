use std::collections::HashMap;

use anyhow::Context;
use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::get_int_rounding;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Language {
    official: OfficialLanguages,
    unofficial: UnofficialLanguages,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfficialLanguages {
    pub english_only: u64,
    pub english_and_french: u64,
    pub neither_english_or_french: u64,
    pub french_only: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnofficialLanguages(pub HashMap<String, u64>);

pub fn get_languages(
    sheet: &Range<Data>,
) -> anyhow::Result<Language> {
    Ok(
        Language {
            official: get_official_languages(sheet)?,
            unofficial: get_unofficial_languages(sheet)?,
        }
    )
}

pub fn get_official_languages(sheet: &Range<Data>) -> anyhow::Result<OfficialLanguages> {
    assert_eq!(
        sheet.get_value((126, 0)),
        Some(&Data::String("LANGUAGES".to_owned()))
    );
    assert_matches!(sheet.get_value((128, 0)), Some(Data::String(str)) if str.trim() == "English only");
    assert_matches!(sheet.get_value((129, 0)), Some(Data::String(str)) if str.trim() == "English and French");
    assert_matches!(sheet.get_value((130, 0)), Some(Data::String(str)) if str.trim() == "Neither English nor French");
    assert_matches!(sheet.get_value((131, 0)), Some(Data::String(str)) if str.trim() == "French only");
    Ok(OfficialLanguages {
        english_only: get_int_rounding(sheet.get_value((128, 1)))
            .context("No or invalid entry for English only")? as u64,
        english_and_french: get_int_rounding(sheet.get_value((129, 1)))
            .context("No or invalid entry for English and French")?
            as u64,
        neither_english_or_french: get_int_rounding(sheet.get_value((130, 1)))
            .context("No or invalid entry for Neither English nor French")?
            as u64,
        french_only: get_int_rounding(sheet.get_value((131, 1)))
            .context("No or invalid entry for French only")? as u64,
    })
}

pub fn get_unofficial_languages(sheet: &Range<Data>) -> anyhow::Result<UnofficialLanguages> {
    let mut map = HashMap::<String, u64>::new();
    assert_eq!(
        sheet.get_value((135, 0)),
        Some(&Data::String(
            "Knowledge of Non-official Languages Spoken (Top 30)".to_owned()
        ))
    );
    for row in sheet.range((136, 0), (165, 1)).rows() {
        match (row[0].clone(), row[1].clone()) {
            (Data::String(language), Data::Int(count)) => {
                assert!(count.is_positive());
                map.insert(language, count as u64);
            }
            (Data::String(language), Data::Float(count)) => {
                assert!(count.is_sign_positive());
                assert_eq!(count.fract(), 0.0);
                map.insert(language, count as u64);
            }
            (bad_lang_type, bad_count_type) => {
                return Err(anyhow::Error::msg(format!(
                    "Bad language and/or count type. Expected a String and Int/Float, got {:#?} and {:#?}",
                    bad_lang_type, bad_count_type
                )));
            }
        }
    }
    map.insert("Other".into(), get_int_rounding(sheet.get_value((166, 1))).context("Other Languages invalid or empty")? as u64);
    Ok(UnofficialLanguages(map))
}
