//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::{ellipsoid::{Ellipsoid}, CoordTransform, PseudoSerialize, DbContstruct};
use std::f64::consts::{FRAC_PI_4, FRAC_PI_2};

#[derive(Copy, Clone, Debug)]
pub struct LambertConic2SPParams {
    /// longitude of false origin
    lon_orig: f64,
    /// latitude of false origin
    lat_orig: f64,
    /// latitude of 1st standard parallel
    lat_p1: f64,
    /// latitude of 2nd standard parallel
    lat_p2: f64,
    /// easting at false origin
    false_e: f64,
    /// easting at false northing
    false_n: f64
}

impl LambertConic2SPParams {
    pub fn new(lon_orig: f64, lat_orig: f64, lat_p1: f64, lat_p2: f64, false_e: f64, false_n: f64) -> Self {
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

pub struct LambertConic1SPParams {
        /// longitude of false origin
        lon_orig: f64,
        /// latitude of false origin
        lat_orig: f64,
        /// latitude of natural origin
        lat_nat_orig: f64,
        /// scale factor at natural origin
        k_nat_orig: f64,
        /// false easting
        false_e: f64,
        /// false northing
        false_n: f64
}

impl LambertConic1SPParams {
    pub fn new(lon_orig: f64, lat_orig: f64, lat_nat_orig: f64, k_nat_orig: f64, false_e: f64, false_n: f64) -> Self {
        Self {
            lat_orig,
            lon_orig,
            lat_nat_orig,
            k_nat_orig,
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
    pub fn lat_nat_orig(&self) -> f64 {
        self.lat_nat_orig
    }

    /// Get latitude of 2nd standard parallel.
    pub fn k_nat_orig(&self) -> f64 {
        self.k_nat_orig
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
        let n;
        let F;
        let r_F;
        if params.lat_p1() == params.lat_p2() {
            let m_O = params.lat_p1().cos() / (1f64 - ell.e_squared() * params.lat_p1().sin().powi(2)).sqrt();
    
            let t_O = (FRAC_PI_4 - params.lat_p1() / 2f64).tan() / ((1f64 - ell.e() * params.lat_p1().sin())/(1f64 + ell.e() * params.lat_p1().sin())).powf(ell.e() / 2f64);
            n = params.lat_p1().sin();
            F = m_O / (n * t_O.powf(n));
            r_F = ell.a() * F * t_O.powf(n);
        } else {
            let m1 = params.lat_p1().cos() / (1f64 - ell.e_squared() * params.lat_p1().sin().powi(2)).sqrt();
            let m2 = params.lat_p2().cos() / (1f64 - ell.e_squared() * params.lat_p2().sin().powi(2)).sqrt();
    
            let t1 = (FRAC_PI_4 - params.lat_p1() / 2f64).tan() / ((1f64 - ell.e() * params.lat_p1().sin())/(1f64 + ell.e() * params.lat_p1().sin())).powf(ell.e() / 2f64);
            let t2 = (FRAC_PI_4 - params.lat_p2() / 2f64).tan() / ((1f64 - ell.e() * params.lat_p2().sin())/(1f64 + ell.e() * params.lat_p2().sin())).powf(ell.e() / 2f64);
            let t_F = (FRAC_PI_4 - params.lat_orig() / 2f64).tan() / ((1f64 - ell.e() * params.lat_orig().sin())/(1f64 + ell.e() * params.lat_orig().sin())).powf(ell.e() / 2f64);
            n = (m1.ln() - m2.ln()) / (t1.ln() - t2.ln());
            F = m1 / (n * t1.powf(n));
            r_F = ell.a() * F * t_F.powf(n);
        }
        Self{
            ellipsoid_e: ell.e(),
            ellipsoid_a: ell.a(),

            lon_orig: params.lon_orig(),
            lat_orig: params.lat_orig(),
            
            false_e: params.false_e(),
            false_n: params.false_n(),

            n,
            r_F,
            F
        }
    }

}

impl CoordTransform for LambertConic2SPConversion {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – May 2022
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        
        let t = (FRAC_PI_4 - latitude / 2f64).tan() / ((1f64 - self.ellipsoid_e * latitude.sin())/(1f64 + self.ellipsoid_e * latitude.sin())).powf(self.ellipsoid_e / 2f64);

        let theta = self.n * (longitude - self.lon_orig);

        let r = self.ellipsoid_a * self.F * t.powf(self.n);
        (
            self.false_e + r * theta.sin()
        ,
            self.false_n + self.r_F - r * theta.cos()
        )
    }
    
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – May 2022
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let theta_ = (self.n.signum() * (easting - self.false_e)).atan2(self.n.signum() * (self.r_F - (northing - self.false_n)));
        let r_ = self.n.signum() * ((easting - self.false_e).powi(2) + (self.r_F - (northing - self.false_n)).powi(2)).sqrt();
        let t_ = (r_ / (self.ellipsoid_a * self.F)).powf(1f64 / self.n);
        let mut phi = FRAC_PI_2 - 2.0 * (t_.atan());
        for _ in 0..Self::MAX_ITERATIONS {
            phi = FRAC_PI_2 - 2.0 * (t_ * ((1f64 - self.ellipsoid_e * phi.sin()) / (1f64 + self.ellipsoid_e * phi.sin())).powf(self.ellipsoid_e / 2f64)).atan()
        }
        (
            theta_ / self.n + self.lon_orig,
            phi
        )
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
    fn lambert_conic_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378160.0, 298.25);
        let params = LambertConic2SPParams::new(
            145f64.to_radians(),
            37f64.to_radians(),
            36f64.to_radians(),
            38f64.to_radians(),
            2_500_000.0,
            4_500_000.0
        );

        let converter = LambertConic2SPConversion::new(&ell, &params);
        let (easting, northing) = converter.from_deg(144.75, 37.75);

        let (lon, lat) = converter.to_deg(easting, northing);
        // TODO: do some reasonable assertions here
    }
}