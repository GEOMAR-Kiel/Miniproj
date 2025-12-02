//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::{DbContstruct, PseudoSerialize, ellipsoid::Ellipsoid, types::GetterContstruct};

#[derive(Copy, Clone, Debug)]
pub struct AlbersEqualAreaParams {
    /// longitude of false origin
    lon_orig: f64,
    /// latitude of false origin
    lat_orig: f64,
    /// latitude of first standard parallel
    lat_sp1: f64,
    /// latitude of second standard parallel
    lat_sp2: f64,
    /// easting at false origin
    false_e: f64,
    /// northing at false origin
    false_n: f64,
}

impl AlbersEqualAreaParams {
    pub const fn new(
        lon_orig: f64,
        lat_orig: f64,
        lat_sp1: f64,
        lat_sp2: f64,
        false_e: f64,
        false_n: f64,
    ) -> Self {
        Self {
            lon_orig,
            lat_orig,
            lat_sp1,
            lat_sp2,
            false_e,
            false_n,
        }
    }

    /// Get longitude of false origin in radians.
    pub fn lon_orig(&self) -> f64 {
        self.lon_orig
    }

    /// Get latitude of false origin in radians.
    pub fn lat_orig(&self) -> f64 {
        self.lat_orig
    }

    /// Get latitude of first standard parallel in radians.
    pub fn lat_sp1(&self) -> f64 {
        self.lat_sp1
    }

    /// Get latitude of second standard parallel in radians.
    pub fn lat_sp2(&self) -> f64 {
        self.lat_sp2
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

/// Lambert Azimuthal Equal Area coordinate operation (EPSG:9820)
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct AlbersEqualAreaProjection {
    pub false_e: f64,
    pub false_n: f64,
    pub lon_orig: f64,
    pub ellipsoid_e: f64,
    pub ellipsoid_e_sq: f64,
    pub ellipsoid_a: f64,
    pub C: f64,
    pub n: f64,
    pub rho_O: f64,
    pub beta_fac_sin2: f64,
    pub beta_fac_sin4: f64,
    pub beta_fac_sin6: f64,
}

impl AlbersEqualAreaProjection {
    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &AlbersEqualAreaParams) -> Self {
        dbg!(ell.e());
        dbg!(ell.e_squared());
        let alpha_O = Self::alpha(ell.e_squared(), params.lat_orig(), ell.e());
        dbg!(alpha_O);
        let alpha_1 = Self::alpha(ell.e_squared(), params.lat_sp1(), ell.e());
        dbg!(alpha_1);
        let alpha_2 = Self::alpha(ell.e_squared(), params.lat_sp2(), ell.e());
        dbg!(alpha_2);
        let m1 = params.lat_sp1().cos()
            / (1f64 - ell.e_squared() * params.lat_sp1().sin().powi(2)).sqrt();
        dbg!(m1);
        let m2 = params.lat_sp2().cos()
            / (1f64 - ell.e_squared() * params.lat_sp2().sin().powi(2)).sqrt();
        dbg!(m2);
        let n = (m1.powi(2) - m2.powi(2)) / (alpha_2 - alpha_1);
        dbg!(n);
        let C = m1.powi(2) + n * alpha_1;
        dbg!(C);
        let rho_O = (ell.a() * (C - n * alpha_O).sqrt()) / n;
        dbg!(rho_O);

        let beta_fac_sin2 = ell.e_squared() / 3f64
            + 31f64 * ell.e_squared().powi(2) / 180f64
            + 517f64 * ell.e_squared().powi(3) / 5040f64;
        let beta_fac_sin4 =
            23f64 * ell.e_squared().powi(2) / 360f64 + 251f64 * ell.e_squared().powi(3) / 3708f64;
        let beta_fac_sin6 = 761f64 * ell.e_squared().powi(3) / 45360f64;

        Self {
            false_e: params.false_e(),
            false_n: params.false_n(),
            lon_orig: params.lon_orig(),
            ellipsoid_e: ell.e(),
            ellipsoid_e_sq: ell.e_squared(),
            ellipsoid_a: ell.a(),
            n,
            C,
            rho_O,
            beta_fac_sin2,
            beta_fac_sin4,
            beta_fac_sin6,
        }
    }

    //#[inline]
    fn alpha(e_sq: f64, phi: f64, e: f64) -> f64 {
        (1f64 - e_sq)
            * ((phi.sin() / (1f64 - e_sq * phi.sin().powi(2)))
                - (1f64 / (2f64 * e)) * ((1f64 - e * phi.sin()) / (1f64 + e * phi.sin())).ln())
    }
}

impl crate::types::Projection for AlbersEqualAreaProjection {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn rad_to_projected(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        let alpha = Self::alpha(self.ellipsoid_e_sq, latitude, self.ellipsoid_e);
        dbg!(alpha);
        let theta = self.n * (longitude - self.lon_orig);
        dbg!(theta);
        let rho = (self.ellipsoid_a * (self.C - self.n * alpha).sqrt()) / self.n;
        dbg!(rho);
        (
            self.false_e + (rho * theta.sin()),
            self.false_n + self.rho_O - (rho * theta.cos()),
        )
    }

    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    ///
    /// The approximation for latitude isn't very precise (6 decimal digits)
    #[allow(non_snake_case)]
    fn projected_to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let theta_: f64 = ((easting - self.false_e) * self.n.signum())
            .atan2((self.rho_O - (northing - self.false_n)) * self.n.signum());
        dbg!(theta_);
        let rho_ = ((easting - self.false_e).powi(2)
            + (self.rho_O - (northing - self.false_n)).powi(2))
        .sqrt();
        dbg!(rho_);
        let alpha_ = (self.C - (rho_.powi(2) * self.n.powi(2) / self.ellipsoid_a.powi(2))) / self.n;
        dbg!(alpha_);
        let beta_ = (alpha_
            / (1f64
                - ((1f64 - self.ellipsoid_e_sq) / (2f64 * self.ellipsoid_e))
                    * ((1f64 - self.ellipsoid_e) / (1f64 + self.ellipsoid_e)).ln()))
        .asin();
        dbg!(beta_);
        let lat = beta_
            + (2f64 * beta_).sin() * self.beta_fac_sin2
            + (4f64 * beta_).sin() * self.beta_fac_sin4
            + (6f64 * beta_).sin() * self.beta_fac_sin6;
        let lon = self.lon_orig + theta_ / self.n;
        (lon, lat)
    }
}

impl PseudoSerialize for AlbersEqualAreaProjection {
    fn to_constructed(&self) -> String {
        format!(
            r"AlbersEqualAreaProjection{{
    false_e: {}f64,
    false_n: {}f64,
    lon_orig: {}f64,
    ellipsoid_e: {}f64,
    ellipsoid_e_sq: {}f64,
    ellipsoid_a: {}f64,
    C: {}f64,
    n: {}f64,
    rho_O: {}f64,
    beta_fac_sin2: {}f64,
    beta_fac_sin4: {}f64,
    beta_fac_sin6: {}f64
}}",
            self.false_e,
            self.false_n,
            self.lon_orig,
            self.ellipsoid_e,
            self.ellipsoid_e_sq,
            self.ellipsoid_a,
            self.C,
            self.n,
            self.rho_O,
            self.beta_fac_sin2,
            self.beta_fac_sin4,
            self.beta_fac_sin6
        )
    }
}

impl DbContstruct for AlbersEqualAreaProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        /*
        ImplementedProjection::new(
            9820,
            &[8802, 8801, 8806, 8807],
            "AlbersEqualAreaParams",
            "AlbersEqualAreaProjection"
        )
        */
        let params = AlbersEqualAreaParams::new(
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

impl GetterContstruct for AlbersEqualAreaProjection {
    fn with_db_getter<G>(mut getter: G, ellipsoid: &Ellipsoid) -> Option<Self>
    where
        G: FnMut(u32) -> Option<f64>,
    {
        let params = AlbersEqualAreaParams::new(
            getter(8822)?,
            getter(8821)?,
            getter(8823)?,
            getter(8824)?,
            getter(8826)?,
            getter(8827)?,
        );
        Some(Self::new(ellipsoid, &params))
    }
}

pub fn direct_projection(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    AlbersEqualAreaProjection::from_database_params(params, &ell).to_constructed()
}

#[cfg(test)]
mod tests {

    use crate::albers_equal_area::*;
    use crate::ellipsoid::Ellipsoid;
    use crate::types::*;

    // TODO: While passing the round-trip, this test does not match what is given in the EPSG Guidance Note 7-2, May 22.
    #[test]
    fn albers_equal_area_consistency_north() {
        let ell = Ellipsoid::from_a_f_inv(6378137.00, 298.2572221);
        let params = AlbersEqualAreaParams::new(
            -1.72787596,
            0.48578331,
            0.49538262,
            0.52854388,
            1000000.000,
            1000000.000,
        );

        let projection = AlbersEqualAreaProjection::new(&ell, &params);
        let easting_goal = 1466493.492;
        let northing_goal = 702903.006;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        eprintln!("lon: {lon}");
        eprintln!("lat: {lat}");
        let (easting, northing) = projection.deg_to_projected(lon, lat);
        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.001);

        assert!((northing - northing_goal).abs() < 0.001);
    }

    #[test]
    fn albers_equal_area_consistency_south() {
        let ell = Ellipsoid::from_a_f_inv(6378160.0, 298.25);
        let params = AlbersEqualAreaParams::new(
            -60f64.to_radians(),
            -32f64.to_radians(),
            -5f64.to_radians(),
            -42f64.to_radians(),
            0.0,
            0.0,
        );

        let projection = AlbersEqualAreaProjection::new(&ell, &params);
        let easting_goal = 1408623.196;
        let northing_goal = 1507641.482;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        eprintln!("lon: {lon}");
        eprintln!("lat: {lat}");
        let (easting, northing) = projection.deg_to_projected(lon, lat);
        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.001);

        assert!((northing - northing_goal).abs() < 0.001);
    }
}
