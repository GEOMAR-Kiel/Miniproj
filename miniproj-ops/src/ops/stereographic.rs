//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::f64::consts::FRAC_PI_2;

use crate::{ellipsoid::Ellipsoid, DbContstruct, PseudoSerialize};

#[derive(Copy, Clone, Debug)]
pub struct PolarStereographicAParams {
    /// longitude of natural origin
    lon_orig: f64,
    /// latitude of natural origin
    lat_orig: f64,
    /// scale factor at natural origin
    k_orig: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64
}

impl PolarStereographicAParams {

    pub const fn new(lon_orig: f64, lat_orig: f64, k_orig: f64, false_e: f64, false_n: f64) -> Self {
        Self {
            lat_orig,
            lon_orig,
            k_orig,
            false_e,
            false_n
        }
    }


    /// longitude of natural origin, radians
    pub fn lon_orig(&self) -> f64 {
        self.lon_orig
    }

    /// latitude of natural origin, radians
    pub fn lat_orig(&self) -> f64 {
        self.lat_orig
    }

    /// scale factor at natural origin
    pub fn k_orig(&self) -> f64 {
        self.k_orig
    }

    /// false easting
    pub fn false_e(&self) -> f64 {
        self.false_e
    }

    /// false northing
    pub fn false_n(&self) -> f64 {
        self.false_n
    }
}

/// Polar Stereographic coordinate operation.
#[derive(Copy, Clone, Debug)]
pub struct PolarStereographicAConversion {
    pub t_rho_factor: f64,

    pub phi_2_chi_sin_summand_factor: f64,
    pub phi_4_chi_sin_summand_factor: f64,
    pub phi_6_chi_sin_summand_factor: f64,
    pub phi_8_chi_sin_summand_factor: f64,

    //params: &'b PolarStereographicAParams,
    pub lat_orig: f64,
    pub lon_orig: f64,
    pub false_e: f64,
    pub false_n: f64,
    //ell: &'a Ellipsoid,
    pub ell_e: f64
}

impl PolarStereographicAConversion{
    pub fn new(ell: &Ellipsoid, params: &PolarStereographicAParams) -> Self {
        let t_rho_factor = ((1.0 + ell.e()).powf(1.0 + ell.e()) * (1.0 - ell.e()).powf(1.0 - ell.e())).sqrt() / (2.0 * ell.a() * params.k_orig());
        let phi_2_chi_sin_summand_factor = ell.e_squared() / 2.0 + 5.0 * ell.e_squared().powi(2) / 24.0 + ell.e_squared().powi(3) / 12.0 + 13.0 * ell.e_squared().powi(4) / 360.0;
        let phi_4_chi_sin_summand_factor = 7.0 * ell.e_squared().powi(2) / 48.0 + 29.0 * ell.e_squared().powi(3) / 240.0 + ell.e_squared().powi(4) / 11520.0;
        let phi_6_chi_sin_summand_factor = 7.0 * ell.e_squared().powi(3) / 120.0 + 81.0 * ell.e_squared().powi(4) / 1120.0;
        let phi_8_chi_sin_summand_factor = 4279.0 * ell.e_squared().powi(4) / 161280.0;
        Self {
            t_rho_factor,
            phi_2_chi_sin_summand_factor,
            phi_4_chi_sin_summand_factor,
            phi_6_chi_sin_summand_factor,
            phi_8_chi_sin_summand_factor,

            lat_orig: params.lat_orig(),
            lon_orig: params.lon_orig(),
            false_e: params.false_e(),
            false_n: params.false_n(),
            
            ell_e: ell.e(),
        }
    }
}

impl crate::traits::CoordTransform for PolarStereographicAConversion {
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        if self.lat_orig < 0.0 { // North Pole Case
            let t = f64::tan(std::f64::consts::FRAC_PI_4 - latitude / 2.0) * ((1.0 + self.ell_e * latitude.sin())  / (1.0 - self.ell_e * latitude.sin())).powf(self.ell_e / 2.0);
            let rho = t / self.t_rho_factor;
            (
                self.false_e + rho * f64::sin(longitude - self.lon_orig)
            ,
                self.false_n - rho * f64::cos(longitude - self.lon_orig)
            )
        } else {    // South Pole Case
            let t = f64::tan(std::f64::consts::FRAC_PI_4 + latitude / 2.0) / ((1.0 + self.ell_e * latitude.sin())  / (1.0 - self.ell_e * latitude.sin())).powf(self.ell_e / 2.0);
            let rho = t / self.t_rho_factor;
            (
                self.false_e + rho * f64::sin(longitude - self.lon_orig)
            ,
                self.false_n + rho * f64::cos(longitude - self.lon_orig)
            )
        }
        
    }

    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let rho_ = ((easting - self.false_e).powi(2) + (northing - self.false_n).powi(2)).sqrt();
        let t_ = rho_ * self.t_rho_factor;
        let chi = if self.lat_orig < 0.0 { // North Pole Case
            FRAC_PI_2 - 2.0 * t_.atan()
        } else { // South Pole Case
            2.0 * t_.atan() - FRAC_PI_2
        };
        let phi = chi +
            self.phi_2_chi_sin_summand_factor * (2.0 * chi).sin() +
            self.phi_4_chi_sin_summand_factor * (4.0 * chi).sin() +
            self.phi_6_chi_sin_summand_factor * (6.0 * chi).sin() + 
            self.phi_8_chi_sin_summand_factor * (8.0 * chi).sin();
        let lambda = /*if easting == self.false_e { //this appears wrong to me so it's commented out. @ me if you think it's right tho.
            self.lat_orig
        } else*/ if self.lat_orig < 0.0 { // North Pole Case
            self.lon_orig + (easting - self.false_e).atan2(self.false_n - northing)
        } else { // South Pole Case
            self.lon_orig + (easting - self.false_e).atan2(northing - self.false_n)
        };
        (lambda, phi)
    }
}
impl DbContstruct for PolarStereographicAConversion {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = PolarStereographicAParams::new(
            params.iter().find_map(|(c, v)| if *c == 8802{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8801{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8805{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8806{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8807{Some(*v)}else{None}).unwrap(),
        );
        Self::new(ellipsoid, &params)
    }
}
impl PseudoSerialize for PolarStereographicAConversion {
    fn to_constructed(&self) -> String {
        format!(
r"PolarStereographicAConversion{{
    t_rho_factor: f64::from_bits(0x{:x}),
    phi_2_chi_sin_summand_factor: f64::from_bits(0x{:x}),
    phi_4_chi_sin_summand_factor: f64::from_bits(0x{:x}),
    phi_6_chi_sin_summand_factor: f64::from_bits(0x{:x}),
    phi_8_chi_sin_summand_factor: f64::from_bits(0x{:x}),
    
    lat_orig: f64::from_bits(0x{:x}),
    lon_orig: f64::from_bits(0x{:x}),
    false_e: f64::from_bits(0x{:x}),
    false_n: f64::from_bits(0x{:x}),

    ell_e: f64::from_bits(0x{:x})
}}",
            self.t_rho_factor.to_bits(),
            self.phi_2_chi_sin_summand_factor.to_bits(),
            self.phi_4_chi_sin_summand_factor.to_bits(),
            self.phi_6_chi_sin_summand_factor.to_bits(),
            self.phi_8_chi_sin_summand_factor.to_bits(),

            self.lat_orig.to_bits(),
            self.lon_orig.to_bits(),
            self.false_e.to_bits(),
            self.false_n.to_bits(),

            self.ell_e.to_bits()
        )
    }
}
pub fn direct_conversion_a(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    PolarStereographicAConversion::from_database_params(params, &ell).to_constructed()
}

#[cfg(test)]
mod tests {

    use crate::stereographic::*;
    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

    use assert_float_eq::*;

    #[test]
    fn polar_stereographic_a_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let params = PolarStereographicAParams::new(
            0.0f64.to_radians(),
            -90.0f64.to_radians(),
            0.994,
            2_000_000.0,
            2_000_000.0
        );

        let converter = PolarStereographicAConversion::new(&ell, &params);
        let (easting, northing) = converter.from_deg(44.0, 73.0);
        let (lon, lat) = converter.to_deg(easting, northing);
        //assert_eq!((easting, northing), (3320416.75, 632668.43));
        assert_eq!((lon, lat), (44.0, 73.0));
    }
}