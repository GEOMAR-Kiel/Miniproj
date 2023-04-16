//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::{ellipsoid::{Ellipsoid}, CoordTransform, PseudoSerialize, DbContstruct};
use std::f64::consts::FRAC_PI_4;

#[derive(Copy, Clone, Debug)]
pub struct LambertConic2SPParams {
    /// longitude of natural origin
    lon_orig: f64,
    /// latitude of natural origin
    lat_orig: f64,
    /// latitude of 1st standard parallel
    lat_p1: f64,
    /// latitude of 2nd standard parallel
    lat_p2: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64
}

impl LambertConic2SPParams {

    pub fn new(lon_orig: f64, lat_orig: f64, lat_p1: f64, lat_p2: f64, false_e: f64, false_n: f64) -> Self {
        assert_eq!(lat_p1, lat_p2);
        assert_ne!(lat_p1, lat_p2);
        Self {
            lat_orig,
            lon_orig,
            lat_p1,
            lat_p2,
            false_e,
            false_n
        }
    }

    /// Get longitude of false origin, radians.
    pub fn lon_orig(&self) -> f64 {
        self.lon_orig
    }

    /// Get latitude of false origin, radians.
    pub fn lat_orig(&self) -> f64 {
        self.lat_orig
    }

    /// Get latitude of 1st standard parallel.
    pub fn lat_p1(&self) -> f64 {
        self.lat_p1
    }

    /// Get latitude of 2nd standard parallel.
    pub fn lat_p2(&self) -> f64 {
        self.lat_p2
    }

    /// Get easting at false origin.
    pub fn false_e(&self) -> f64 {
        self.false_e
    }

    /// Get northing at false origin.
    pub fn false_n(&self) -> f64 {
        self.false_n
    }
}

/// Transverse Mercator coordinate operation (EPSG:9807).
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct LambertConic2SPConversion {
    pub ellipsoid_e: f64,
    pub ellipsoid_a: f64,

    pub lon_orig: f64,
    pub lat_orig: f64,

    pub lat_p1: f64,
    pub lat_p2: f64,

    pub false_e: f64,
    pub false_n: f64,

    pub n: f64,
    pub r_F: f64,
    pub F: f64,
}

impl LambertConic2SPConversion {
    const MAX_ITERATIONS: usize = 4;

    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &LambertConic2SPParams) -> Self {
        let m1 = params.lat_p1().cos() / (1f64 - ell.e().powi(2) * params.lat_p1().sin().powi(2)).sqrt();
        let m2 = params.lat_p2().cos() / (1f64 - ell.e().powi(2) * params.lat_p2().sin().powi(2)).sqrt();

        let t1 = (FRAC_PI_4 - params.lat_p1() / 2f64).tan() / ((1f64 - ell.e() * params.lat_p1().sin())/(1f64 + ell.e() * params.lat_p1().sin())).powf(ell.e() / 2f64);
        let t2 = (FRAC_PI_4 - params.lat_p2() / 2f64).tan() / ((1f64 - ell.e() * params.lat_p2().sin())/(1f64 + ell.e() * params.lat_p2().sin())).powf(ell.e() / 2f64);
        let t_F = (FRAC_PI_4 - params.lat_orig() / 2f64).tan() / ((1f64 - ell.e() * params.lat_orig().sin())/(1f64 + ell.e() * params.lat_orig().sin())).powf(ell.e() / 2f64);
        assert!(t1 > 0f64);
        assert!(t2 > 0f64);
        assert!(m1 > 0f64);
        assert!(m2 > 0f64);
        assert_ne!(t1, t2);
        let n = (m1.ln() - m2.ln()) / (t1.ln() - t2.ln());
        let F = m1 / (n * t1.powf(n));
        let r_F = ell.a() * F * t_F.powf(n);


        Self{
            ellipsoid_e: ell.e(),
            ellipsoid_a: ell.a(),

            lon_orig: params.lon_orig(),
            lat_orig: params.lat_orig(),
            
            lat_p1: params.lat_p1(),
            lat_p2: params.lat_p2(),
            
            false_e: params.false_e(),
            false_n: params.false_n(),

            n,
            r_F,
            F
        }
    }
}

impl CoordTransform for LambertConic2SPConversion {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        
        let t = (FRAC_PI_4 - self.lat_orig / 2f64).tan() / ((1f64 - self.ellipsoid_e * self.lat_orig.sin())/(1f64 + self.ellipsoid_e * self.lat_orig.sin())).powf(self.ellipsoid_e / 2f64);

        let theta = self.n * (longitude - self.lon_orig);

        let r = self.ellipsoid_a * self.F * t.powf(self.n);
        (
            self.false_e + r * theta.sin()
        ,
            self.false_n + self.r_F - r * theta.cos()
        )
    }
    
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        todo!()
    }

}

impl PseudoSerialize for LambertConic2SPConversion {
    fn to_constructed(&self) -> String {
        format!(
r"LambertConic2SPConversion{{
    ellipsoid_e: f64::from_bits(0x{:x}),
    ellipsoid_a: f64::from_bits(0x{:x}),
    lon_orig: f64::from_bits(0x{:x}),
    lat_orig: f64::from_bits(0x{:x}),
    lat_p1: f64::from_bits(0x{:x}),
    lat_p2: f64::from_bits(0x{:x}),
    false_e: f64::from_bits(0x{:x}),
    false_n: f64::from_bits(0x{:x}),
    n: f64::from_bits(0x{:x}),
    r_F: f64::from_bits(0x{:x}),
    F: f64::from_bits(0x{:x}),
}}",
            self.ellipsoid_e.to_bits(),
            self.ellipsoid_a.to_bits(),
            self.lon_orig.to_bits(),
            self.lat_orig.to_bits(),
            self.lat_p1.to_bits(),
            self.lat_p2.to_bits(),
            self.false_e.to_bits(),
            self.false_n.to_bits(),
            self.n.to_bits(),
            self.r_F.to_bits(),
            self.F.to_bits()
        )
    }
}

impl DbContstruct for LambertConic2SPConversion {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = LambertConic2SPParams::new(
            params.iter().find_map(|(c, v)| if *c == 8822{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8821{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8823{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8824{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8826{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8827{Some(*v)}else{None}).unwrap(),
        );
        Self::new(ellipsoid, &params)
    }
}

pub fn direct_conversion_2sp(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    LambertConic2SPConversion::from_database_params(params, &ell).to_constructed()
}
#[cfg(test)]
mod tests {

    use crate::lambert_conic_conformal::*;
    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

    use assert_float_eq::*;

    #[test]
    fn transverse_mercator_consistency() {
        let wgs_84_ellipsoid = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let utm_32_n = LambertConic2SPParams::new(
            9.0f64.to_radians(),
            0.0f64.to_radians(),
            0.9996,
            500_000.0,
            0.0
        );

        let converter = LambertConic2SPConversion::new(&wgs_84_ellipsoid, &utm_32_n);
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