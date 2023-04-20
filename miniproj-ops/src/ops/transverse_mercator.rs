//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::{ellipsoid::{Ellipsoid}, Projection, PseudoSerialize, DbContstruct};

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

    /// Get longitude of natural origin, radians.
    pub fn lon_orig(&self) -> f64 {
        self.lon_orig
    }

    /// Get latitude of natural origin, radians.
    pub fn lat_orig(&self) -> f64 {
        self.lat_orig
    }

    /// Get scale factor at natural origin.
    pub fn k_orig(&self) -> f64 {
        self.k_orig
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
pub struct TransverseMercatorProjection {
    pub ellipsoid_e: f64,

    pub lon_orig: f64,
    pub false_e: f64,
    pub false_n: f64,
    pub k_orig: f64,

    pub B: f64,
    pub h_1: f64,
    pub h_2: f64,
    pub h_3: f64,
    pub h_4: f64,
    pub M_orig: f64,

    pub h_1_: f64,
    pub h_2_: f64,
    pub h_3_: f64,
    pub h_4_: f64
}

impl TransverseMercatorProjection {
    const MAX_ITERATIONS: usize = 4;

    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &TransverseMercatorParams) -> Self {
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
            ellipsoid_e: ell.e(),
            lon_orig: params.lon_orig(),
            false_e: params.false_e(),
            false_n: params.false_n(),
            k_orig: params.k_orig(),


            B,
            h_1,
            h_2,
            h_3,
            h_4,
            M_orig,

            h_1_,
            h_2_,
            h_3_,
            h_4_,
        }
    }
}

impl Projection for TransverseMercatorProjection {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn from_rad(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        let Q = latitude.tan().asinh() - (self.ellipsoid_e * f64::atanh(self.ellipsoid_e * latitude.sin()));
        let beta = Q.sinh().atan();
        let eta_0 = f64::atanh(beta.cos() * f64::sin(longitude - self.lon_orig));
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
            self.false_e + self.k_orig * self.B * eta
        ,
            self.false_n + self.k_orig * (self.B * xi - self.M_orig)
        )
    }
    
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let eta_ = (easting - self.false_e) / (self.B * self.k_orig);
        let xi_ = ((northing - self.false_n) + self.k_orig * self.M_orig) / (self.B * self.k_orig);

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
        let mut Q__ = Q_ + (self.ellipsoid_e * f64::atanh(self.ellipsoid_e * Q_.tanh()));
        for _ in 0..Self::MAX_ITERATIONS {
            Q__ = Q_ + (self.ellipsoid_e * f64::atanh(self.ellipsoid_e * Q__.tanh()));
        };

        (
            self.lon_orig + f64::asin(eta_0_.tanh() / beta_.cos())
        ,
            Q__.sinh().atan()
        )
    }

}

impl PseudoSerialize for TransverseMercatorProjection {
    fn to_constructed(&self) -> String {
        format!(
r"TransverseMercatorProjection{{
    ellipsoid_e: f64::from_bits(0x{:x}),
    lon_orig: f64::from_bits(0x{:x}),
    false_e: f64::from_bits(0x{:x}),
    false_n: f64::from_bits(0x{:x}),
    k_orig: f64::from_bits(0x{:x}),

    B: f64::from_bits(0x{:x}),
    h_1: f64::from_bits(0x{:x}),
    h_2: f64::from_bits(0x{:x}),
    h_3: f64::from_bits(0x{:x}),
    h_4: f64::from_bits(0x{:x}),
    M_orig: f64::from_bits(0x{:x}),

    h_1_: f64::from_bits(0x{:x}),
    h_2_: f64::from_bits(0x{:x}),
    h_3_: f64::from_bits(0x{:x}),
    h_4_: f64::from_bits(0x{:x}),
}}",
            self.ellipsoid_e.to_bits(),
            self.lon_orig.to_bits(),
            self.false_e.to_bits(),
            self.false_n.to_bits(),
            self.k_orig.to_bits(),

            self.B.to_bits(),
            self.h_1.to_bits(),
            self.h_2.to_bits(),
            self.h_3.to_bits(),
            self.h_4.to_bits(),
            self.M_orig.to_bits(),
            
            self.h_1_.to_bits(),
            self.h_2_.to_bits(),
            self.h_3_.to_bits(),
            self.h_4_.to_bits()
        )
    }
}

impl DbContstruct for TransverseMercatorProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        /*
        ImplementedProjection::new(
            9807,
            // lon   lat     k     e     n
            &[8802, 8801, 8805, 8806, 8807],
            "TransverseMercatorParams",
            "TransverseMercatorProjection"
        ),
        */
        let params = TransverseMercatorParams::new(
            params.iter().find_map(|(c, v)| if *c == 8802{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8801{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8805{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8806{Some(*v)}else{None}).unwrap(),
            params.iter().find_map(|(c, v)| if *c == 8807{Some(*v)}else{None}).unwrap(),
        );
        Self::new(ellipsoid, &params)
    }
}

pub fn direct_projection(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    TransverseMercatorProjection::from_database_params(params, &ell).to_constructed()
}
#[cfg(test)]
mod tests {

    use crate::transverse_mercator::*;
    use crate::traits::*;
    use crate::ellipsoid::Ellipsoid;

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

        let projection = TransverseMercatorProjection::new(&wgs_84_ellipsoid, &utm_32_n);
        let easting_goal = 577274.99;
        let northing_goal = 69740.50;
        let (lon, lat) = projection.to_deg(easting_goal, northing_goal);
        let (easting, northing) = projection.from_deg(lon, lat);

        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.01);

        assert!((northing - northing_goal).abs() < 0.01);
    }
}