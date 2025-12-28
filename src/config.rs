use rust_decimal::Decimal;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub filter_rule: FilterRule,
}

#[derive(Deserialize)]
pub struct FilterRule {
    pub premium_threshold: Decimal,
    pub purchase_limit: u64,
}

pub fn load_config() -> Config {
    let content = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
}
