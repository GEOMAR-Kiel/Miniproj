//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use crate::{ellipsoid::Ellipsoid, DbContstruct, Projection, PseudoSerialize};

#[derive(Copy, Clone, Debug)]
pub struct PopVisPseudoMercatorParams {
    /// longitude of natural origin
    lon_orig: f64,
    /// latitude of natural origin
    lat_orig: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64,
}

impl PopVisPseudoMercatorParams {
    pub const fn new(lon_orig: f64, lat_orig: f64, false_e: f64, false_n: f64) -> Self {
        Self {
            lat_orig,
            lon_orig,
            false_e,
            false_n,
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
pub struct PopVisPseudoMercatorProjection {
    pub false_e: f64,
    pub false_n: f64,
    pub ellipsoid_a: f64,
    pub lon_orig: f64,
}

impl PopVisPseudoMercatorProjection {
    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &PopVisPseudoMercatorParams) -> Self {
        Self {
            ellipsoid_a: ell.a(),
            lon_orig: params.lon_orig(),
            false_e: params.false_e(),
            false_n: params.false_n(),
        }
    }
}

impl Projection for PopVisPseudoMercatorProjection {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn rad_to_projected(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        (
            self.false_e + self.ellipsoid_a * (longitude - self.lon_orig),
            self.false_n + self.ellipsoid_a * (FRAC_PI_4 + latitude / 2f64).tan().ln(),
        )
    }

    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn projected_to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let D = (self.false_n - northing) / self.ellipsoid_a;
        (
            ((easting - self.false_e) / self.ellipsoid_a) + self.lon_orig,
            FRAC_PI_2 - 2.0 * D.exp().atan(),
        )
    }
}

impl PseudoSerialize for PopVisPseudoMercatorProjection {
    fn to_constructed(&self) -> String {
        format!(
r"PopVisPseudoMercatorProjection{{
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

impl DbContstruct for PopVisPseudoMercatorProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = PopVisPseudoMercatorParams::new(
            params
                .iter()
                .find_map(|(c, v)| if *c == 8802 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8801 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8806 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8807 { Some(*v) } else { None })
                .unwrap(),
        );
        Self::new(ellipsoid, &params)
    }
}

pub fn direct_projection(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    PopVisPseudoMercatorProjection::from_database_params(params, &ell).to_constructed()
}
#[cfg(test)]
mod tests {

    use crate::ellipsoid::Ellipsoid;
    use crate::traits::*;

    use super::PopVisPseudoMercatorParams;
    use super::PopVisPseudoMercatorProjection;

    #[test]
    fn popvis_mercator_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let params =
            PopVisPseudoMercatorParams::new(0.0f64.to_radians(), 0.0f64.to_radians(), 0.0, 0.0);

        let projection = PopVisPseudoMercatorProjection::new(&ell, &params);
        let easting_goal = -11169055.58;
        let northing_goal = 2800000.00;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        let (easting, northing) = projection.deg_to_projected(lon, lat);

        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.01);

        assert!((northing - northing_goal).abs() < 0.01);
    }
}
