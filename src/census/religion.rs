use std::collections::HashMap;

use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Religion(pub HashMap<String, u64>);

pub fn get_religion(sheet: &Range<Data>) -> anyhow::Result<Religion> {
    assert_matches!(sheet.get_value((326, 0)), Some(Data::String(str)) if str.trim() == "RELIGION");
    let mut map = HashMap::new();
    for row in sheet.range((328, 0), (350, 1)).rows() {
        match (row[0].clone(), row[1].clone()) {
            (Data::String(religion), Data::Int(count)) => {
                assert!(count.is_positive());
                map.insert(religion.trim().to_owned(), count as u64);
            },
            (Data::String(religion), Data::Float(count)) => {
                assert!(count.is_sign_positive());
                assert_eq!(count.fract(), 0.0);
                map.insert(religion.trim().to_owned(), count as u64);
            },
            (invalid_religion, invalid_count) => {
                return Err(anyhow::Error::msg(format!("Expected a String and Int/Float. Got {:#?} and {:#?}", invalid_religion, invalid_count)));
            }
        }
    }
    Ok(Religion(map))
}