use anyhow::Ok;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::{assert_test_ranges, population::{Population, get_row_int_population}};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MainTransportation {
    pub personal_vehicle_driver: Population,
    pub public_transit: Population,
    pub personal_vehicle_passenger: Population,
    pub walk: Population,
    pub bicycle: Population,
    pub other: Population,
}

pub fn get_transportation(sheet: &Range<Data>) -> anyhow::Result<MainTransportation> {
    assert_test_ranges(
        sheet,
        [
            ((476, 0), "Car, truck or van - as a driver"),
            ((477, 0), "Public transit"),
            ((478, 0), "Car, truck or van - as a passenger"),
            ((479, 0), "Walk"),
            ((480, 0), "Bicycle"),
            ((481, 0), "Other method"),
        ]
        .into_iter(),
    );
    Ok(MainTransportation {
        personal_vehicle_driver: get_row_int_population(sheet, 476).unwrap(),
        public_transit: get_row_int_population(sheet, 477).unwrap(),
        personal_vehicle_passenger: get_row_int_population(sheet, 478).unwrap(),
        walk: get_row_int_population(sheet, 479).unwrap(),
        bicycle: get_row_int_population(sheet, 480).unwrap(),
        other: get_row_int_population(sheet, 481).unwrap(),
    })
}
