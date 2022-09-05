// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base = speed as f64 * 221.0;
    if speed <= 4 {
        base * 1.0
    } else if speed <= 8 {
        base * 0.9
    } else {
        base * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
