//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::ellipsoid::Ellipsoid;

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
pub struct PolarStereographicAConversion<'a, 'b> {
    params: &'b PolarStereographicAParams,
    ell: &'a Ellipsoid,


}
unsafe impl<'a, 'b> Send for PolarStereographicAConversion<'a, 'b> {}
unsafe impl<'a, 'b> Sync for PolarStereographicAConversion<'a, 'b> {}

impl<'a, 'b> PolarStereographicAConversion<'a, 'b> {
    pub fn new(ell: &'a Ellipsoid, params: &'b PolarStereographicAParams) -> Self {
        Self {
            ell,
            params
        }
    }
}

impl crate::traits::CoordTransform for PolarStereographicAConversion<'_, '_> {
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        if self.params.lat_orig() < 0.0 { // North Pole Case
            let t = f64::tan(std::f64::consts::FRAC_PI_4 + latitude / 2.0) / (((1.0 + self.ell.e() * latitude.sin())  / (1.0 - self.ell.e() * latitude.sin())).powf(self.ell.e() / 2.0));
            let rho = 2.0 * self.ell.a() * self.params.k_orig() * t / (((1.0 + self.ell.e()).powf(1.0 + self.ell.e()) * (1.0 - self.ell.e()).powf(1.0 - self.ell.e())).sqrt());
            (
                self.params.false_e() + rho * f64::sin(longitude - self.params.lon_orig())
            ,
                self.params.false_n() - rho * f64::cos(longitude - self.params.lon_orig())
            )
        } else {    // South Pole Case
            let t = f64::tan(std::f64::consts::FRAC_PI_4 - latitude / 2.0) / (((1.0 + self.ell.e() * latitude.sin())  / (1.0 - self.ell.e() * latitude.sin())).powf(self.ell.e() / 2.0));
            let rho = 2.0 * self.ell.a() * self.params.k_orig() * t / (((1.0 + self.ell.e()).powf(1.0 + self.ell.e()) * (1.0 - self.ell.e()).powf(1.0 - self.ell.e())).sqrt());
            (
                self.params.false_e() + rho * f64::sin(longitude - self.params.lon_orig())
            ,
                self.params.false_n() + rho * f64::cos(longitude - self.params.lon_orig())
            )
        }
        
    }

    fn to_rad(&self, _easting: f64, _northing: f64) -> (f64, f64) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::stereographic::*;
    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

    use assert_float_eq::*;

    #[test]
    fn polar_stereographic_a_consistency() {
        let wgs_84_ellipsoid = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let utm_32_n = PolarStereographicAParams::new(
            9.0f64.to_radians(),
            0.0f64.to_radians(),
            0.9996,
            500_000.0,
            0.0
        );

        let converter = PolarStereographicAConversion::new(&wgs_84_ellipsoid, &utm_32_n);
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