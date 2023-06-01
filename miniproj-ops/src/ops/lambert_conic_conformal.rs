//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::{ellipsoid::Ellipsoid, DbContstruct, Projection, PseudoSerialize};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

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
    false_n: f64,
}

impl LambertConic2SPParams {
    pub fn new(
        lon_orig: f64,
        lat_orig: f64,
        lat_p1: f64,
        lat_p2: f64,
        false_e: f64,
        false_n: f64,
    ) -> Self {
        Self {
            lat_orig,
            lon_orig,
            lat_p1,
            lat_p2,
            false_e,
            false_n,
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

/// EPSG:9802: Lambert Conic Conformal (2SP) .
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct LambertConic2SPProjection {
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

impl LambertConic2SPProjection {
    const MAX_ITERATIONS: usize = 4;

    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &LambertConic2SPParams) -> Self {
        let n;
        let F;
        let r_F;
        if params.lat_p1() == params.lat_p2() {
            let m_O = params.lat_p1().cos()
                / (1f64 - ell.e_squared() * params.lat_p1().sin().powi(2)).sqrt();

            let t_O = (FRAC_PI_4 - params.lat_p1() / 2f64).tan()
                / ((1f64 - ell.e() * params.lat_p1().sin())
                    / (1f64 + ell.e() * params.lat_p1().sin()))
                .powf(ell.e() / 2f64);
            n = params.lat_p1().sin();
            F = m_O / (n * t_O.powf(n));
            r_F = ell.a() * F * t_O.powf(n);
        } else {
            let m1 = params.lat_p1().cos()
                / (1f64 - ell.e_squared() * params.lat_p1().sin().powi(2)).sqrt();
            let m2 = params.lat_p2().cos()
                / (1f64 - ell.e_squared() * params.lat_p2().sin().powi(2)).sqrt();

            let t1 = (FRAC_PI_4 - params.lat_p1() / 2f64).tan()
                / ((1f64 - ell.e() * params.lat_p1().sin())
                    / (1f64 + ell.e() * params.lat_p1().sin()))
                .powf(ell.e() / 2f64);
            let t2 = (FRAC_PI_4 - params.lat_p2() / 2f64).tan()
                / ((1f64 - ell.e() * params.lat_p2().sin())
                    / (1f64 + ell.e() * params.lat_p2().sin()))
                .powf(ell.e() / 2f64);
            let t_F = (FRAC_PI_4 - params.lat_orig() / 2f64).tan()
                / ((1f64 - ell.e() * params.lat_orig().sin())
                    / (1f64 + ell.e() * params.lat_orig().sin()))
                .powf(ell.e() / 2f64);
            n = (m1.ln() - m2.ln()) / (t1.ln() - t2.ln());
            F = m1 / (n * t1.powf(n));
            r_F = ell.a() * F * t_F.powf(n);
        }
        Self {
            ellipsoid_e: ell.e(),
            ellipsoid_a: ell.a(),

            lon_orig: params.lon_orig(),
            lat_orig: params.lat_orig(),

            false_e: params.false_e(),
            false_n: params.false_n(),

            n,
            r_F,
            F,
        }
    }
}

impl Projection for LambertConic2SPProjection {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – May 2022
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn rad_to_projected(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        let t = (FRAC_PI_4 - latitude / 2f64).tan()
            / ((1f64 - self.ellipsoid_e * latitude.sin())
                / (1f64 + self.ellipsoid_e * latitude.sin()))
            .powf(self.ellipsoid_e / 2f64);

        let theta = self.n * (longitude - self.lon_orig);

        let r = self.ellipsoid_a * self.F * t.powf(self.n);
        (
            self.false_e + r * theta.sin(),
            self.false_n + self.r_F - r * theta.cos(),
        )
    }

    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – May 2022
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn projected_to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let theta_ = (self.n.signum() * (easting - self.false_e))
            .atan2(self.n.signum() * (self.r_F - (northing - self.false_n)));
        let r_ = self.n.signum()
            * ((easting - self.false_e).powi(2) + (self.r_F - (northing - self.false_n)).powi(2))
                .sqrt();
        let t_ = (r_ / (self.ellipsoid_a * self.F)).powf(1f64 / self.n);
        let mut phi = FRAC_PI_2 - 2.0 * (t_.atan());
        for _ in 0..Self::MAX_ITERATIONS {
            phi = FRAC_PI_2
                - 2.0
                    * (t_
                        * ((1f64 - self.ellipsoid_e * phi.sin())
                            / (1f64 + self.ellipsoid_e * phi.sin()))
                        .powf(self.ellipsoid_e / 2f64))
                    .atan()
        }
        (theta_ / self.n + self.lon_orig, phi)
    }
}

impl PseudoSerialize for LambertConic2SPProjection {
    fn to_constructed(&self) -> String {
        format!(
            r"LambertConic2SPProjection{{
    ellipsoid_e: {}f64,
    ellipsoid_a: {}f64,
    lon_orig: {}f64,
    lat_orig: {}f64,
    false_e: {}f64,
    false_n: {}f64,
    n: {}f64,
    r_F: {}f64,
    F: {}f64,
}}",
            self.ellipsoid_e,
            self.ellipsoid_a,
            self.lon_orig,
            self.lat_orig,
            self.false_e,
            self.false_n,
            self.n,
            self.r_F,
            self.F
        )
    }
}

impl DbContstruct for LambertConic2SPProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = LambertConic2SPParams::new(
            params
                .iter()
                .find_map(|(c, v)| if *c == 8822 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8821 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8823 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8824 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8826 { Some(*v) } else { None })
                .unwrap(),
            params
                .iter()
                .find_map(|(c, v)| if *c == 8827 { Some(*v) } else { None })
                .unwrap(),
        );
        Self::new(ellipsoid, &params)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LambertConic1SPAParams {
    /// longitude of false origin
    lon_nat_orig: f64,
    /// latitude of false origin
    lat_nat_orig: f64,
    /// scale factor at natural origin
    k_nat_orig: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64,
}

impl LambertConic1SPAParams {
    pub fn new(
        lon_nat_orig: f64,
        lat_nat_orig: f64,
        k_nat_orig: f64,
        false_e: f64,
        false_n: f64,
    ) -> Self {
        Self {
            lon_nat_orig,
            lat_nat_orig,
            k_nat_orig,
            false_e,
            false_n,
        }
    }

    /// Get longitude of natural origin, radians.
    pub fn lon_nat_orig(&self) -> f64 {
        self.lon_nat_orig
    }

    /// Get latitude of natural origin, radians.
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

/// EPSG:9801: Lambert Conic Conformal (2SP).
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct LambertConic1SPAProjection {
    pub false_e: f64,
    pub false_n: f64,

    pub r_O: f64,
    pub lon_O: f64,

    pub n: f64,
    pub t_r_fac: f64,
    pub ellipsoid_e: f64,
}

impl LambertConic1SPAProjection {
    const MAX_ITERATIONS: usize = 4;

    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &LambertConic1SPAParams) -> Self {
        let m_O = params.lat_nat_orig().cos()
            / (1f64 - ell.e_squared() * params.lat_nat_orig().sin().powi(2)).sqrt();
        let t_O = (FRAC_PI_4 - params.lat_nat_orig() / 2f64).tan()
            / ((1f64 - ell.e() * params.lat_nat_orig().sin())
                / (1f64 + ell.e() * params.lat_nat_orig().sin()))
            .powf(ell.e() / 2f64);
        let n = params.lat_nat_orig.sin();
        let F = m_O / (n * t_O.powf(n));
        let r_O = ell.a() * F * t_O.powf(n) * params.k_nat_orig();
        Self {
            false_e: params.false_e(),
            false_n: params.false_n(),

            r_O,
            lon_O: params.lon_nat_orig(),
            n,
            t_r_fac: ell.a() * F * params.k_nat_orig(),
            ellipsoid_e: ell.e(),
        }
    }
}

impl Projection for LambertConic1SPAProjection {
    fn projected_to_rad(&self, x: f64, y: f64) -> (f64, f64) {
        let theta_ = (self.n.signum() * (x - self.false_e))
            .atan2(self.n.signum() * (self.r_O - (y - self.false_n)));
        let r_ = self.n.signum()
            * ((x - self.false_e).powi(2) + (self.r_O - (y - self.false_n)).powi(2)).sqrt();
        let t_ = (r_ / self.t_r_fac).powf(1f64 / self.n);
        let mut phi = FRAC_PI_2 - 2f64 * t_.atan();
        for _ in 0..Self::MAX_ITERATIONS {
            phi = FRAC_PI_2
                - 2f64
                    * (t_
                        * ((1f64 - self.ellipsoid_e * phi.sin())
                            / (1f64 + self.ellipsoid_e * phi.sin()))
                        .powf(self.ellipsoid_e / 2f64))
                    .atan();
        }
        (theta_ / self.n + self.lon_O, phi)
    }

    fn rad_to_projected(&self, lon: f64, lat: f64) -> (f64, f64) {
        let t = (FRAC_PI_4 - lat / 2f64).tan()
            / ((1f64 - self.ellipsoid_e * lat.sin()) / (1f64 + self.ellipsoid_e * lat.sin()))
                .powf(self.ellipsoid_e / 2f64);
        let r = self.t_r_fac * t.powf(self.n);
        let theta = self.n * (lon - self.lon_O);
        (
            self.false_e + r * theta.sin(),
            self.false_n + self.r_O - r * theta.cos(),
        )
    }
}

impl PseudoSerialize for LambertConic1SPAProjection {
    fn to_constructed(&self) -> String {
        format!(
            "LambertConic1SPAProjection {{
    false_e: {}f64,
    false_n: {}f64,
    r_O: {}f64,
    lon_O: {}f64,
    n: {}f64,
    t_r_fac: {}f64,
    ellipsoid_e: {}f64
}}
",
            self.false_e,
            self.false_n,
            self.r_O,
            self.lon_O,
            self.n,
            self.t_r_fac,
            self.ellipsoid_e
        )
    }
}

impl DbContstruct for LambertConic1SPAProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = LambertConic1SPAParams::new(
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
                .find_map(|(c, v)| if *c == 8805 { Some(*v) } else { None })
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

pub fn direct_projection_2sp(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    LambertConic2SPProjection::from_database_params(params, &ell).to_constructed()
}

pub fn direct_projection_1sp_a(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    LambertConic1SPAProjection::from_database_params(params, &ell).to_constructed()
}

#[cfg(test)]
mod tests {

    use crate::ellipsoid::Ellipsoid;
    use crate::lambert_conic_conformal::*;
    use crate::traits::*;

    #[test]
    fn lambert_conic_2sp_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378160.0, 298.25);
        let params = LambertConic2SPParams::new(
            145f64.to_radians(),
            37f64.to_radians(),
            36f64.to_radians(),
            38f64.to_radians(),
            2_500_000.0,
            4_500_000.0,
        );

        let projection = LambertConic2SPProjection::new(&ell, &params);
        let easting_goal = 2477968.963;
        let northing_goal = 4416742.535;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        let (easting, northing) = projection.deg_to_projected(lon, lat);

        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.001);

        assert!((northing - northing_goal).abs() < 0.001);
    }

    #[test]
    fn lambert_conic_1sp_a_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378206.400, 294.97870);
        let params = LambertConic1SPAParams::new(
            18f64.to_radians(),
            -77f64.to_radians(),
            1.0,
            2_500_000.0,
            1_500_000.0,
        );

        let projection = LambertConic1SPAProjection::new(&ell, &params);
        let easting_goal = 255966.58;
        let northing_goal = 142493.51;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        let (easting, northing) = projection.deg_to_projected(lon, lat);

        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.001);

        assert!((northing - northing_goal).abs() < 0.001);
    }
}
