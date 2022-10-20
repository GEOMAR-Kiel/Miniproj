//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::ellipsoid::Ellipsoid;

#[derive(Copy, Clone, Debug)]
pub struct LambertAzimuthalEqualAreaParams {
    /// longitude of natural origin
    lon_orig: f64,
    /// latitude of natural origin
    lat_orig: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64
}

impl LambertAzimuthalEqualAreaParams {

    pub const fn new(lon_orig: f64, lat_orig: f64, false_e: f64, false_n: f64) -> Self {
        Self {
            lat_orig,
            lon_orig,
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

    /// false easting
    pub fn false_e(&self) -> f64 {
        self.false_e
    }

    /// false northing
    pub fn false_n(&self) -> f64 {
        self.false_n
    }
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct LambertAzimuthalEqualAreaConversion<'a, 'b> {
    params: &'b LambertAzimuthalEqualAreaParams,
    ell: &'a Ellipsoid,

    //q_O: f64,
    q_P: f64,
    beta_O: f64,
    R_q: f64,
    D: f64

}
unsafe impl<'a, 'b> Send for LambertAzimuthalEqualAreaConversion<'a, 'b> {}
unsafe impl<'a, 'b> Sync for LambertAzimuthalEqualAreaConversion<'a, 'b> {}

impl<'a, 'b> LambertAzimuthalEqualAreaConversion<'a, 'b> {

    #[allow(non_snake_case)]
    pub fn new(ell: &'a Ellipsoid, params: &'b LambertAzimuthalEqualAreaParams) -> Self {
        
        let q_P = (1.0 - ell.e().powi(2)) * 
        (
            (1.0 / (1.0 - ell.e().powi(2))) - 
            ((0.5 / ell.e()) * f64::ln((1.0 - ell.e()) / (1.0 + ell.e())))
        );

        
        let q_O = (1.0 - ell.e().powi(2)) * 
        (
            (params.lat_orig().sin() / (1.0 - ell.e().powi(2) * params.lat_orig().sin().powi(2))) - 
            (
                (0.5 / ell.e()) * 
                f64::ln(
                    (1.0 - ell.e() * params.lat_orig().sin()) / 
                    (1.0 + ell.e() * params.lat_orig().sin())
                )
            )
        );

        let beta_O = (q_O / q_P).asin();

        let R_q = ell.a() * (q_P / 2.0).sqrt();

        let D = ell.a() * (params.lat_orig().cos() / (1.0 - ell.e().powi(2) * params.lat_orig().sin().powi(2)).sqrt()) / (R_q * beta_O.cos());

        Self{
            params,
            ell,

            q_P,
            //q_O,
            beta_O,
            R_q,
            D
        }
    }
}

impl crate::traits::CoordTransform for LambertAzimuthalEqualAreaConversion<'_, '_> {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {

        let q = (1.0 - self.ell.e().powi(2)) * 
        (
            (latitude.sin() / (1.0 - self.ell.e().powi(2) * latitude.sin().powi(2))) - 
            (
                (0.5 / self.ell.e()) * 
                f64::ln(
                    (1.0 - self.ell.e() * latitude.sin()) / 
                    (1.0 + self.ell.e() * latitude.sin())
                )
            )
        );

        let beta = (q / self.q_P).asin();

        let B = self.R_q * (2.0 / (1.0 + self.beta_O.sin() * beta.sin() + (self.beta_O.cos() * beta.cos() * (longitude - self.params.lon_orig).cos()))).sqrt();

        (
            self.params.false_e() + ((B * self.D) * (beta.cos() * (longitude - self.params.lon_orig).sin()))
        ,
            self.params.false_n() + (B / self.D) * ((self.beta_O.cos() * beta.sin()) - (self.beta_O.sin() * beta.cos() * (longitude - self.params.lon_orig).cos()))
        )
    }
    
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    ///
    /// The approximation for latitude isn't very precise (6 decimal digits)
    #[allow(non_snake_case)]
    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        
        let rho = (((easting - self.params.false_e())/ self.D).powi(2) + (self.D * (northing - self.params.false_n())).powi(2)).sqrt();

        let C = 2.0 * (rho / 2.0 / self.R_q).asin();

        let beta_ = ((C.cos() * self.beta_O.sin()) + ((self.D * (northing - self.params.false_n()) * C.sin() * self.beta_O.cos()) / rho)).asin();

        (
            self.params.lon_orig() + f64::atan2(
                (easting - self.params.false_e()) * C.sin(),
                self.D * rho * self.beta_O.cos() * C.cos() - self.D.powi(2) * (northing - self.params.false_n) * self.beta_O.sin() * C.sin()
            )
        ,
            beta_ + 
            (
                (self.ell.e().powi(2) / 3.0 + (31.0 / 180.0) * self.ell.e().powi(4) + (517.0 / 5040.0) * self.ell.e().powi(6)) * (beta_ * 2.0).sin() + 
                ((23.0 / 360.0) * self.ell.e().powi(4) + (251.0 / 3780.0) * self.ell.e().powi(6)) * (beta_ * 4.0).sin() + 
                (761.0 / 45360.0) * self.ell.e().powi(6) * (beta_ + 6.0).sin()
            )
        )
    }

}

#[cfg(test)]
mod tests {

    use crate::lambert_azimuthal_equal_area::*;
    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

    use assert_float_eq::*;

    #[test]
    fn lambert_azimuthal_equal_area_consistency() {
        let grs_1980_ellipsoid = Ellipsoid::from_a_f_inv(6378137.0, 298.2572221);
        let etrs_laea = LambertAzimuthalEqualAreaParams::new(
            10.0f64.to_radians(),
            52.0f64.to_radians(),

            4_321_000.0,
            3_210_000.0
        );

        let converter = LambertAzimuthalEqualAreaConversion::new(&grs_1980_ellipsoid, &etrs_laea);
        let lat = 50.0;
        let lon = 5.0;

                let pos = (lon as f64, lat as f64);
                eprintln!("coord: {:#?}", pos);
                let pos_laea = converter.from_deg(pos.0, pos.1);
                eprintln!("laea: {:#?}", pos_laea);
                let pos_2 = converter.to_deg(pos_laea.0, pos_laea.1);
                eprintln!("coord: {:#?}", pos_2);
                assert_f64_near!(pos.0, pos_2.0, 1 << 9);
                assert_f64_near!(pos.1, pos_2.1, 1 << 9);// this one always fails because the approximation in the reverse conversion is so bad
        
    }
}