use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::assert_get_counts;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct VisibleMinorities {
    pub filipino: u64,
    pub south_asian: u64,
    pub black: u64,
    pub chinese: u64,
    pub southeast_asian: u64,
    pub latin_american: u64,
    pub arab: u64,
    pub korean: u64,
    pub west_asian: u64,
    pub japanese: u64,
    pub multiple: u64,
    pub other: u64,
}

pub fn get_visible_minorities(sheet: &Range<Data>) -> anyhow::Result<VisibleMinorities> {
    let mut values = assert_get_counts(
        sheet,
        [
            (202, "Filipino"),
            (203, "South Asian"),
            (204, "Black"),
            (205, "Chinese"),
            (206, "Southeast Asian"),
            (207, "Latin American"),
            (208, "Arab"),
            (209, "Korean"),
            (210, "West Asian"),
            (211, "Japanese"),
            (212, "Multiple visible minorities"),
            (213, "Visible minority not included elsewhere"),
        ].into_iter(),
    );
    Ok(VisibleMinorities {
        filipino: values.next().unwrap()?,
        south_asian: values.next().unwrap()?,
        black: values.next().unwrap()?,
        chinese: values.next().unwrap()?,
        southeast_asian: values.next().unwrap()?,
        latin_american: values.next().unwrap()?,
        arab: values.next().unwrap()?,
        korean: values.next().unwrap()?,
        west_asian: values.next().unwrap()?,
        japanese: values.next().unwrap()?,
        multiple: values.next().unwrap()?,
        other: values.next().unwrap()?,
    })
}
