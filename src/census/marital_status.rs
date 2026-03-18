use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::assert_get_counts;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MaritalStatus {
    pub married: u64,
    pub common_law: u64,
    pub single: u64,
    pub separated: u64,
    pub divorced: u64,
    pub widowed: u64,
}

pub fn get_marital_status(sheet: &Range<Data>) -> anyhow::Result<MaritalStatus> {
    let mut items = assert_get_counts(
        sheet,
        [
            (357, "Married (and not separated)"),
            (358, "Living common law"),
            (360, "Single (never legally married)"),
            (361, "Separated , but still legally married"),
            (362, "Divorced"),
            (363, "Widowed"),
        ]
        .into_iter(),
    );
    Ok(MaritalStatus {
        married: items.next().unwrap()?,
        common_law: items.next().unwrap()?,
        single: items.next().unwrap()?,
        separated: items.next().unwrap()?,
        divorced: items.next().unwrap()?,
        widowed: items.next().unwrap()?,
    })
}
