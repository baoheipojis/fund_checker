use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    filter_rule: FilterRule,
}

#[derive(Deserialize)]
struct FilterRule {
    premium_threshold: f64,
    purchase_limit: u64,
}

fn load_config() -> Config {
    let content = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
}

fn main() {
    let config = load_config();
    println!("溢价率阈值: {}%", config.filter_rule.premium_threshold);
    println!("申购限额: {} 元", config.filter_rule.purchase_limit);
}
