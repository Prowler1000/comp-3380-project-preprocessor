use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::assert_get_counts;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Citizenship {
    pub citizen: u64,
    pub non_citizen: u64,
}

pub fn get_citizenship(sheet: &Range<Data>) -> anyhow::Result<Citizenship> {
    let mut values = assert_get_counts(
        sheet,
        [(222, "Canadian citizens"), (223, "Not Canadian citizens")].into_iter(),
    );
    Ok(Citizenship {
        citizen: values.next().unwrap()?,
        non_citizen: values.next().unwrap()?,
    })
}
