use crate::data::rule::Rule;
use crate::rules;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Rules {
    And { rules: Vec<Rules> },
    Or { rules: Vec<Rules> },
    Luminosity(rules::luminosity::RuleLuminosity),
    DysonRadius(rules::dyson_radius::RuleDysonRadius),
    AverageVeinAmount(rules::average_vein_amount::RuleAverageVeinAmount),
    Spectr(rules::spectr::RuleSpectr),
    TidalLockCount(rules::tidal_lock_count::RuleTidalLockCount),
    OceanType(rules::ocean_type::RuleOceanType),
    StarType(rules::star_type::RuleStarType),
    GasCount(rules::gas_count::RuleGasCount),
    SatelliteCount(rules::satellite_count::RuleSatelliteCount),
    Birth(rules::birth::RuleBirth),
    ThemeId(rules::theme_id::RuleThemeId),
    PlanetCount(rules::planet_count::RulePlanetCount),
}

pub fn sort_rules(rules: Vec<Rules>) -> Vec<Box<dyn Rule + Send>> {
    let mut result: Vec<Box<dyn Rule + Send>> = rules.into_iter().map(transform_rules).collect();
    result.sort_by_key(|rule| rule.get_priority());
    result
}

pub fn transform_rules(r: Rules) -> Box<dyn Rule + Send> {
    match r {
        Rules::And { rules } => Box::new(rules::and::RuleAnd {
            rules: sort_rules(rules),
        }),
        Rules::Or { rules } => Box::new(rules::or::RuleOr {
            rules: sort_rules(rules),
        }),
        Rules::Luminosity(rule) => Box::new(rule),
        Rules::DysonRadius(rule) => Box::new(rule),
        Rules::AverageVeinAmount(rule) => Box::new(rule),
        Rules::Spectr(rule) => Box::new(rule),
        Rules::TidalLockCount(rule) => Box::new(rule),
        Rules::OceanType(rule) => Box::new(rule),
        Rules::StarType(rule) => Box::new(rule),
        Rules::GasCount(rule) => Box::new(rule),
        Rules::SatelliteCount(rule) => Box::new(rule),
        Rules::Birth(rule) => Box::new(rule),
        Rules::ThemeId(rule) => Box::new(rule),
        Rules::PlanetCount(rule) => Box::new(rule),
    }
}
