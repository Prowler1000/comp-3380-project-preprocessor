use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod census;
pub mod geospatial;
pub mod neighbourhood;
pub mod street;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Datapoint<T> {
    NotApplicable,
    Redacted,
    #[serde(untagged)]
    Datapoint(T),
}

pub static LOCATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"((?P<latitude>-?[0-9]{2}\.[0-9]+)\s(?P<longitude>-?[0-9]{2}\.[0-9]+)[,\s]{0,2})",
    )
    .expect("Failed to create location regex")
});