//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

pub fn epsg_9110_to_deg(val: f64) -> f64 {
    let sign = val.signum();
    let a = val.abs();
    let whole_deg = a.trunc();
    let arcmins = (a.fract() * 100f64).trunc();
    let arcsecs = (a.fract() * 100f64).fract() * 100f64;
    sign * (whole_deg + arcmins / 60f64 + arcsecs / 3600f64).to_radians()
}

pub fn string_to_const_name(val: &str) -> String {
    val.chars()
        .map(|c: char| 
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            })
        .collect()
}