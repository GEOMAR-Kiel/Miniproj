//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::f64::consts::{FRAC_PI_4, FRAC_PI_2};

use crate::{ellipsoid::{Ellipsoid}, CoordTransform, PseudoSerialize, DbContstruct};

#[derive(Copy, Clone, Debug)]
pub struct PopVisPseudoMercatorParams {
    /// longitude of natural origin
    lon_orig: f64,
    /// latitude of natural origin
    lat_orig: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64
}

impl PopVisPseudoMercatorParams {

    pub const fn new(lon_orig: f64, lat_orig: f64, false_e: f64, false_n: f64) -> Self {
        Self {
            lat_orig,
            lon_orig,
            false_e,
            false_n
        }
    }

    /// Get longitude of natural origin, radians.
    pub fn lon_orig(&self) -> f64 {
        self.lon_orig
    }

    /// Get latitude of natural origin, radians.
    pub fn lat_orig(&self) -> f64 {
        self.lat_orig
    }

    /// Get false easting.
    pub fn false_e(&self) -> f64 {
        self.false_e
    }

    /// Get false northing.
    pub fn false_n(&self) -> f64 {
        self.false_n
    }
}

/// Transverse Mercator coordinate operation (EPSG:9807).
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct PopVisPseudoMercatorConversion {
    pub false_e: f64,
    pub false_n: f64,
    pub ellipsoid_a: f64,
    pub lon_orig: f64
}

impl PopVisPseudoMercatorConversion {

    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &PopVisPseudoMercatorParams) -> Self {
        Self{
            ellipsoid_a: ell.a(),
            lon_orig: params.lon_orig(),
            false_e: params.false_e(),
            false_n: params.false_n(),
        }
    }
}

impl CoordTransform for PopVisPseudoMercatorConversion {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        (
            self.false_e + self.ellipsoid_a * (longitude - self.lon_orig)
        ,
            self.false_n + self.ellipsoid_a * (FRAC_PI_4 + latitude / 2f64).tan().ln()
        )
    }
    
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let D = (self.false_n - northing) / self.ellipsoid_a;
        (
            ((easting - self.false_e) / self.ellipsoid_a) + self.lon_orig
        ,
            FRAC_PI_2 - 2.0 * D.exp().atan()
        )
    }

}

impl PseudoSerialize for PopVisPseudoMercatorConversion {
    fn to_constructed(&self) -> String {
        format!(
r"PopVisPseudoMercatorConversion{{
    ellipsoid_a: f64::from_bits(0x{:x}),
    lon_orig: f64::from_bits(0x{:x}),
    false_e: f64::from_bits(0x{:x}),
    false_n: f64::from_bits(0x{:x}),
}}",
            self.ellipsoid_a.to_bits(),
            self.lon_orig.to_bits(),
            self.false_e.to_bits(),
            self.false_n.to_bits(),
        )
    }
}

impl DbContstruct for PopVisPseudoMercatorConversion {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = PopVisPseudoMercatorParams::new(
            params.iter().find_map(|(c, v)| if *c == 8802{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8801{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8806{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8807{Some(*v)}else{None}).unwrap(),
        );
        Self::new(ellipsoid, &params)
    }
}

pub fn direct_conversion(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    PopVisPseudoMercatorConversion::from_database_params(params, &ell).to_constructed()
}
#[cfg(test)]
mod tests {

    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

    use assert_float_eq::*;

    use super::PopVisPseudoMercatorConversion;
    use super::PopVisPseudoMercatorParams;

    #[test]
    fn popvis_mercator_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let params = PopVisPseudoMercatorParams::new(
            0.0f64.to_radians(),
            0.0f64.to_radians(),
            0.0,
            0.0,
        );

        let converter = PopVisPseudoMercatorConversion::new(&ell, &params);
        for lon in 6 .. 12 {
            for lat in -80 .. 80 {
                let pos = (lon as f64, lat as f64);
                let pos_utm = converter.from_deg(pos.0, pos.1);
                let pos_2 = converter.to_deg(pos_utm.0, pos_utm.1);
                assert_f64_near!(pos.0, pos_2.0, 256 * 3);
                assert_f64_near!(pos.1, pos_2.1, 256 * 3);
            }
        }
    }
}