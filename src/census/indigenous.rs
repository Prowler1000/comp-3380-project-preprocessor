use anyhow::Context;
use assert_matches::assert_matches;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::get_int_rounding;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Indigenous {
    pub identity: IndigenousIdentity,
    pub ancestry: IndigenousAncestry,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct IndigenousIdentity {
    pub métis: u64,
    pub first_nations: u64,
    pub inuk: u64,
    pub multiple: u64,
    pub other: u64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct IndigenousAncestry {
    first_nations: u64,
    métis: u64,
    inuit: u64,
}

pub fn get_indigenous(sheet: &Range<Data>) -> anyhow::Result<Indigenous> {
    Ok(Indigenous {
        identity: get_indigenous_identity(sheet)?,
        ancestry: get_indigenous_ancestry(sheet)?,
    })
}

pub fn get_indigenous_identity(sheet: &Range<Data>) -> anyhow::Result<IndigenousIdentity> {
    assert_matches!(sheet.get_value((178, 0)), Some(Data::String(str)) if str.trim() == "Métis single identity");
    assert_matches!(sheet.get_value((179, 0)), Some(Data::String(str)) if str.trim() == "First Nations (North American Indian) single identity");
    assert_matches!(sheet.get_value((180, 0)), Some(Data::String(str)) if str.trim() == "Inuk (Inuit) single identity");
    assert_matches!(sheet.get_value((181, 0)), Some(Data::String(str)) if str.trim() == "Multiple Indigenous identities");
    assert_matches!(sheet.get_value((182, 0)), Some(Data::String(str)) if str.trim() == "Aboriginal identities not included elsewhere");
    Ok(IndigenousIdentity {
        métis: get_int_rounding(sheet.get_value((178, 1)))
            .context("Invalid count for Métis single identity")? as u64,
        first_nations: get_int_rounding(sheet.get_value((179, 1)))
            .context("Invalid count for First Nations (North American Indian) single identity")?
            as u64,
        inuk: get_int_rounding(sheet.get_value((180, 1)))
            .context("Invalid count for Inuk (Inuit) single identity")? as u64,
        multiple: get_int_rounding(sheet.get_value((181, 1)))
            .context("Invalid count for Multiple Indigenous identities")? as u64,
        other: get_int_rounding(sheet.get_value((182, 1)))
            .context("Invalid count for Aboriginal identities not included elsewhere")?
            as u64,
    })
}

pub fn get_indigenous_ancestry(sheet: &Range<Data>) -> anyhow::Result<IndigenousAncestry> {
    assert_matches!(sheet.get_value((190, 0)), Some(Data::String(str)) if str.trim() == "First Nations (North American Indian) Indigenous ancestry");
    assert_matches!(sheet.get_value((191, 0)), Some(Data::String(str)) if str.trim() == "Métis ancestry");
    assert_matches!(sheet.get_value((192, 0)), Some(Data::String(str)) if str.trim() == "Inuit ancestry");
    Ok(IndigenousAncestry {
        first_nations: get_int_rounding(sheet.get_value((190, 1))).context(
            "Invalid count for First Nations (North American Indian) Indigenous ancestry",
        )? as u64,
        métis: get_int_rounding(sheet.get_value((191, 1)))
            .context("Invalid count for Métis ancestry")? as u64,
        inuit: get_int_rounding(sheet.get_value((192, 1)))
            .context("Invalid count for Inuit ancestry")? as u64,
    })
}
