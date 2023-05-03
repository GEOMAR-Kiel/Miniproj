/// This file is licensed under EUPL v1.2

/// Converts an EPSG:9110-encoded angle to radians.
/// 
/// The encoding represents degrees, minutes and seconds as decimal digits, 
/// with degrees encoded as the integer part and minutes and seconds as the
/// first and second pair of fractional digits respectively.
/// 
/// This is a highly flawed encoding, as not all decimal numbers with at most
/// 4 fractional digits can be represented as IEEE 754 floating point numbers.
pub fn epsg_9110_to_rad(val: f64) -> f64 {
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