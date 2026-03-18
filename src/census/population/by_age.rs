use anyhow::Context;
use calamine::{Data, Range};
use serde::{Deserialize, Serialize};

use crate::census::{get_int_rounding, population::Population};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PopulationByAge {
    pub range_0_to_4: Population,
    pub range_5_to_9: Population,
    pub range_10_to_14: Population,
    pub range_15_to_19: Population,
    pub range_20_to_24: Population,
    pub range_25_to_29: Population,
    pub range_30_to_34: Population,
    pub range_35_to_39: Population,
    pub range_40_to_44: Population,
    pub range_45_to_49: Population,
    pub range_50_to_54: Population,
    pub range_55_to_59: Population,
    pub range_60_to_64: Population,
    pub range_65_to_69: Population,
    pub range_70_to_74: Population,
    pub range_75_to_79: Population,
    pub range_80_to_84: Population,
    pub range_85_to_89: Population,
    pub range_90_to_94: Population,
    pub range_95_to_99: Population,
    pub range_100_plus: Population,
}

impl PopulationByAge {
    pub fn sum_ages(&self) -> Population {
        self.range_0_to_4 + self.range_5_to_9 + self.range_10_to_14 + self.range_15_to_19 + self.range_20_to_24 + self.range_25_to_29 + self.range_30_to_34 + self.range_35_to_39 + self.range_40_to_44 + self.range_45_to_49 + self.range_50_to_54 + self.range_55_to_59 + self.range_60_to_64 + self.range_65_to_69 + self.range_70_to_74 + self.range_75_to_79 + self.range_80_to_84 + self.range_85_to_89 + self.range_90_to_94 + self.range_95_to_99 + self.range_100_plus
    }
}

pub fn get_population_by_age(range: &Range<Data>) -> anyhow::Result<PopulationByAge> {
    let sub_range = range.range((97, 0), (120, 2));
    assert_eq!(sub_range.get((0, 0)), Some(&Data::String("POPULATION BY AGE".to_owned())));
    let total_male = get_int_rounding(sub_range.get((23, 1))).context("Total male population invalid or empty value")? as u64;
    let total_female = get_int_rounding(sub_range.get((23, 2))).context("Total female population invalid or empty value")? as u64;
    let pop_by_age = PopulationByAge {
        range_0_to_4: get_age_population(&sub_range, 2, 1).context("Failed to get 0-4 population")?,
        range_5_to_9: get_age_population(&sub_range, 3, 1).context("Failed to get 5-9 population")?,
        range_10_to_14: get_age_population(&sub_range, 4, 1).context("Failed to get 10-14 population")?,
        range_15_to_19: get_age_population(&sub_range, 5, 1).context("Failed to get 15-19 population")?,
        range_20_to_24: get_age_population(&sub_range, 6, 1).context("Failed to get 20-24 population")?,
        range_25_to_29: get_age_population(&sub_range, 7, 1).context("Failed to get 24-29 population")?,
        range_30_to_34: get_age_population(&sub_range, 8, 1).context("Failed to get 30-34 population")?,
        range_35_to_39: get_age_population(&sub_range, 9, 1).context("Failed to get 35-39 population")?,
        range_40_to_44: get_age_population(&sub_range, 10, 1).context("Failed to get 40-44 population")?,
        range_45_to_49: get_age_population(&sub_range, 11, 1).context("Failed to get 45-49 population")?,
        range_50_to_54: get_age_population(&sub_range, 12, 1).context("Failed to get 50-54 population")?,
        range_55_to_59: get_age_population(&sub_range, 13, 1).context("Failed to get 55-59 population")?,
        range_60_to_64: get_age_population(&sub_range, 14, 1).context("Failed to get 60-64 population")?,
        range_65_to_69: get_age_population(&sub_range, 15, 1).context("Failed to get 65-69 population")?,
        range_70_to_74: get_age_population(&sub_range, 16, 1).context("Failed to get 70-74 population")?,
        range_75_to_79: get_age_population(&sub_range, 17, 1).context("Failed to get 75-79 population")?,
        range_80_to_84: get_age_population(&sub_range, 18, 1).context("Failed to get 80-84 population")?,
        range_85_to_89: get_age_population(&sub_range, 19, 1).context("Failed to get 85-89 population")?,
        range_90_to_94: get_age_population(&sub_range, 20, 1).context("Failed to get 90-94 population")?,
        range_95_to_99: get_age_population(&sub_range, 21, 1).context("Failed to get 95-99 population")?,
        range_100_plus: get_age_population(&sub_range, 22, 1).context("Failed to get 100+ population")?,
    };
    let summed_pop = pop_by_age.sum_ages();
    if total_male != summed_pop.male {
        return Err(anyhow::Error::msg(format!("Calculated total for male didn't equal recorded total. Counted {}, expected {}", summed_pop.male, total_male)));
    }
    if total_female != summed_pop.female {
        return Err(anyhow::Error::msg(format!("Calculated total for female didn't equal recorded total. Counted {}, expected {}", summed_pop.female, total_female)));
    }
    Ok(pop_by_age)
}

fn get_age_population(range: &Range<Data>, row_offset: usize, column_offset: usize,) -> anyhow::Result<Population> {
    Ok(Population {
        male: get_int_rounding(range.get((row_offset, column_offset))).context("Male population value is invalid or doesn't exist")? as u64,
        female: get_int_rounding(range.get((row_offset, column_offset+1))).context("Female population value is invalid or doesn't exist")? as u64,
    })
}