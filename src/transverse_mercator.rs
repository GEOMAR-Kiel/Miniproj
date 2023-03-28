//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::ellipsoid::Ellipsoid;

#[derive(Copy, Clone, Debug)]
pub struct TransverseMercatorParams {
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

impl TransverseMercatorParams {

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

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct TransverseMercatorConversion<'a, 'b> {
    params: &'b TransverseMercatorParams,
    ell: &'a Ellipsoid,

    B: f64,
    h_1: f64,
    h_2: f64,
    h_3: f64,
    h_4: f64,
    M_orig: f64,

    h_1_: f64,
    h_2_: f64,
    h_3_: f64,
    h_4_: f64
}
unsafe impl<'a, 'b> Send for TransverseMercatorConversion<'a, 'b> {}
unsafe impl<'a, 'b> Sync for TransverseMercatorConversion<'a, 'b> {}

impl<'a, 'b> TransverseMercatorConversion<'a, 'b> {
    const MAX_ITERATIONS: usize = 4;

    #[allow(non_snake_case)]
    pub fn new(ell: &'a Ellipsoid, params: &'b TransverseMercatorParams) -> Self {
        let n = ell.f() / (2.0 - ell.f());
        let B = (ell.a() / (1.0 + n)) * (1.0 + n.powi(2)/4.0 + n.powi(4)/64.0);
    
        let h_1 = n / 2.0 - (2.0 / 3.0) * n.powi(2) + (5.0 / 16.0) * n.powi(3) + (41.0 / 180.0) * n.powi(4);
        let h_2 = (13.0 / 48.0) * n.powi(2) - (3.0 / 5.0) * n.powi(3) + (557.0 / 1440.0) * n.powi(4);
        let h_3 = (61.0 / 240.0) * n.powi(3) - (103.0 / 140.0) * n.powi(4);
        let h_4 = (49561.0 / 161280.0) * n.powi(4);
    
        let M_orig = if params.lat_orig() == 0.0 { 0.0 }
            else if params.lat_orig() == std::f64::consts::FRAC_PI_2 { B * std::f64::consts::FRAC_PI_2 }
            else if params.lat_orig() == - std::f64::consts::FRAC_PI_2 { -B * std::f64::consts::FRAC_PI_2 }
            else {
                let Q_orig = params.lat_orig().tan().asinh() - (ell.e() * f64::atanh(ell.e() * params.lat_orig().sin()));
    
                let beta_orig = Q_orig.sinh().atan();
                let xi_orig_0 = beta_orig;
    
                let xi_orig_1 = h_1 * f64::sin(2.0 * xi_orig_0);
                let xi_orig_2 = h_2 * f64::sin(4.0 * xi_orig_0);
                let xi_orig_3 = h_3 * f64::sin(6.0 * xi_orig_0);
                let xi_orig_4 = h_4 * f64::sin(8.0 * xi_orig_0);
                let xi_orig = xi_orig_0 + xi_orig_1 + xi_orig_2 + xi_orig_3 + xi_orig_4;
                B * xi_orig
            };
        
        let h_1_ = n / 2.0 - (2.0 / 3.0) * n.powi(2) + (37.0 / 96.0) * n.powi(3) - (1.0 / 360.0) * n.powi(4);
        let h_2_ = (1.0 / 48.0) * n.powi(2) + (1.0 / 15.0) * n.powi(3) - (437.0 / 1440.0) * n.powi(4);
        let h_3_ = (17.0 / 480.0) * n.powi(3) - (37.0 / 840.0) * n.powi(4);
        let h_4_ = (4397.0 / 161280.0) * n.powi(4);

        Self{
            params,
            ell,

            B,
            h_1,
            h_2,
            h_3,
            h_4,
            M_orig,

            h_1_,
            h_2_,
            h_3_,
            h_4_
        }
    }
}

impl crate::traits::CoordTransform for TransverseMercatorConversion<'_, '_> {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        let Q = latitude.tan().asinh() - (self.ell.e() * f64::atanh(self.ell.e() * latitude.sin()));
        let beta = Q.sinh().atan();
        let eta_0 = f64::atanh(beta.cos() * f64::sin(longitude - self.params.lon_orig()));
        let xi_0 = f64::asin(beta.sin() * eta_0.cosh());

        let xi_1 = self.h_1 * f64::sin(2.0 * xi_0) * f64::cosh(2.0 * eta_0);
        let xi_2 = self.h_2 * f64::sin(4.0 * xi_0) * f64::cosh(4.0 * eta_0);
        let xi_3 = self.h_3 * f64::sin(6.0 * xi_0) * f64::cosh(6.0 * eta_0);
        let xi_4 = self.h_4 * f64::sin(8.0 * xi_0) * f64::cosh(8.0 * eta_0);
        let xi = xi_0 + xi_1 + xi_2 + xi_3 + xi_4;

        let eta_1 = self.h_1 * f64::cos(2.0 * xi_0) * f64::sinh(2.0 * eta_0);
        let eta_2 = self.h_2 * f64::cos(4.0 * xi_0) * f64::sinh(4.0 * eta_0);
        let eta_3 = self.h_3 * f64::cos(6.0 * xi_0) * f64::sinh(6.0 * eta_0);
        let eta_4 = self.h_4 * f64::cos(8.0 * xi_0) * f64::sinh(8.0 * eta_0);
        let eta = eta_0 + eta_1 + eta_2 + eta_3 + eta_4;

        (
            self.params.false_e() + self.params.k_orig() * self.B * eta
        ,
            self.params.false_n() + self.params.k_orig() * (self.B * xi - self.M_orig)
        )
    }
    
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let eta_ = (easting - self.params.false_e()) / (self.B * self.params.k_orig());
        let xi_ = ((northing - self.params.false_n()) + self.params.k_orig() * self.M_orig) / (self.B * self.params.k_orig());

        let xi_1_ = self.h_1_ * f64::sin(2.0 * xi_) * f64::cosh(2.0 * eta_);
        let xi_2_ = self.h_2_ * f64::sin(4.0 * xi_) * f64::cosh(4.0 * eta_);
        let xi_3_ = self.h_3_ * f64::sin(6.0 * xi_) * f64::cosh(6.0 * eta_);
        let xi_4_ = self.h_4_ * f64::sin(8.0 * xi_) * f64::cosh(8.0 * eta_);
        let xi_0_ = xi_ - (xi_1_ + xi_2_ + xi_3_ + xi_4_);

        let eta_1_ = self.h_1_ * f64::cos(2.0 * xi_) * f64::sinh(2.0 * eta_);
        let eta_2_ = self.h_2_ * f64::cos(4.0 * xi_) * f64::sinh(4.0 * eta_);
        let eta_3_ = self.h_3_ * f64::cos(6.0 * xi_) * f64::sinh(6.0 * eta_);
        let eta_4_ = self.h_4_ * f64::cos(8.0 * xi_) * f64::sinh(8.0 * eta_);
        let eta_0_ = eta_ - (eta_1_ + eta_2_ + eta_3_ + eta_4_);

        let beta_ = f64::asin(xi_0_.sin() / eta_0_.cosh());
        let Q_ = beta_.tan().asinh();
        let mut Q__ = Q_ + (self.ell.e() * f64::atanh(self.ell.e() * Q_.tanh()));
        for _ in 0..Self::MAX_ITERATIONS {
            Q__ = Q_ + (self.ell.e() * f64::atanh(self.ell.e() * Q__.tanh()));
        };

        (
            self.params.lon_orig() + f64::asin(eta_0_.tanh() / beta_.cos())
        ,
            Q__.sinh().atan()
        )
    }

}

#[cfg(test)]
mod tests {

    use crate::transverse_mercator::*;
    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

    use assert_float_eq::*;

    #[test]
    fn transverse_mercator_consistency() {
        let wgs_84_ellipsoid = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let utm_32_n = TransverseMercatorParams::new(
            9.0f64.to_radians(),
            0.0f64.to_radians(),
            0.9996,
            500_000.0,
            0.0
        );

        let converter = TransverseMercatorConversion::new(&wgs_84_ellipsoid, &utm_32_n);
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