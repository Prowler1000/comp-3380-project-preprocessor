use std::sync::LazyLock;

use regex::Regex;


pub static LOCATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"((?P<latitude>-?[0-9]{2}\.[0-9]+)\s(?P<longitude>-?[0-9]{2}\.[0-9]+)[,\s]{0,2})",
    )
    .expect("Failed to create location regex")
});