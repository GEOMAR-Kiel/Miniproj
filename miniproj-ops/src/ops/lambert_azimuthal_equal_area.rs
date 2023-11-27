//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::{ellipsoid::Ellipsoid, traits::GetterContstruct, DbContstruct, PseudoSerialize};

#[derive(Copy, Clone, Debug)]
pub struct LambertAzimuthalEqualAreaParams {
    /// longitude of natural origin
    lon_orig: f64,
    /// latitude of natural origin
    lat_orig: f64,
    /// false easting
    false_e: f64,
    /// false northing
    false_n: f64,
}

impl LambertAzimuthalEqualAreaParams {
    pub const fn new(lon_orig: f64, lat_orig: f64, false_e: f64, false_n: f64) -> Self {
        Self {
            lat_orig,
            lon_orig,
            false_e,
            false_n,
        }
    }

    /// Get longitude of natural origin in radians.
    pub fn lon_orig(&self) -> f64 {
        self.lon_orig
    }

    /// Get latitude of natural origin in radians.
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

/// Lambert Azimuthal Equal Area coordinate operation (EPSG:9820)
#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct LambertAzimuthalEqualAreaProjection {
    pub lon_orig: f64,
    pub false_e: f64,
    pub false_n: f64,
    pub ellipsoid_e: f64,
    pub ellipsoid_e_squared: f64,

    //q_O: f64,
    pub q_P: f64,
    pub beta_O: f64,
    pub R_q: f64,
    pub D: f64,
}

impl LambertAzimuthalEqualAreaProjection {
    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &LambertAzimuthalEqualAreaParams) -> Self {
        let q_P = (1.0 - ell.e_squared())
            * ((1.0 / (1.0 - ell.e_squared()))
                - ((0.5 / ell.e()) * f64::ln((1.0 - ell.e()) / (1.0 + ell.e()))));

        let q_O = (1.0 - ell.e_squared())
            * ((params.lat_orig().sin()
                / (1.0 - ell.e_squared() * params.lat_orig().sin().powi(2)))
                - ((0.5 / ell.e())
                    * f64::ln(
                        (1.0 - ell.e() * params.lat_orig().sin())
                            / (1.0 + ell.e() * params.lat_orig().sin()),
                    )));

        let beta_O = (q_O / q_P).asin();

        let R_q = ell.a() * (q_P / 2.0).sqrt();

        let D = ell.a()
            * (params.lat_orig().cos()
                / (1.0 - ell.e_squared() * params.lat_orig().sin().powi(2)).sqrt())
            / (R_q * beta_O.cos());

        Self {
            lon_orig: params.lon_orig(),
            false_e: params.false_e(),
            false_n: params.false_n(),
            ellipsoid_e: ell.e(),
            ellipsoid_e_squared: ell.e_squared(),

            q_P,
            //q_O,
            beta_O,
            R_q,
            D,
        }
    }
}

impl crate::traits::Projection for LambertAzimuthalEqualAreaProjection {
    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    #[allow(non_snake_case)]
    fn rad_to_projected(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        let q = (1.0 - self.ellipsoid_e_squared)
            * ((latitude.sin() / (1.0 - self.ellipsoid_e_squared * latitude.sin().powi(2)))
                - ((0.5 / self.ellipsoid_e)
                    * f64::ln(
                        (1.0 - self.ellipsoid_e * latitude.sin())
                            / (1.0 + self.ellipsoid_e * latitude.sin()),
                    )));

        let beta = (q / self.q_P).asin();

        let B = self.R_q
            * (2.0
                / (1.0
                    + self.beta_O.sin() * beta.sin()
                    + (self.beta_O.cos() * beta.cos() * (longitude - self.lon_orig).cos())))
            .sqrt();

        (
            self.false_e + ((B * self.D) * (beta.cos() * (longitude - self.lon_orig).sin())),
            self.false_n
                + (B / self.D)
                    * ((self.beta_O.cos() * beta.sin())
                        - (self.beta_O.sin() * beta.cos() * (longitude - self.lon_orig).cos())),
        )
    }

    /// as per IOGP Publication 373-7-2 – Geomatics Guidance Note number 7, part 2 – March 2020
    /// longitude & latitude in radians
    ///
    /// The approximation for latitude isn't very precise (6 decimal digits)
    #[allow(non_snake_case)]
    fn projected_to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let rho = (((easting - self.false_e) / self.D).powi(2)
            + (self.D * (northing - self.false_n)).powi(2))
        .sqrt();

        let C = 2.0 * (rho / 2.0 / self.R_q).asin();

        let beta_ = ((C.cos() * self.beta_O.sin())
            + ((self.D * (northing - self.false_n) * C.sin() * self.beta_O.cos()) / rho))
            .asin();

        (
            self.lon_orig
                + f64::atan2(
                    (easting - self.false_e) * C.sin(),
                    self.D * rho * self.beta_O.cos() * C.cos()
                        - self.D.powi(2) * (northing - self.false_n) * self.beta_O.sin() * C.sin(),
                ),
            beta_
                + ((self.ellipsoid_e_squared / 3.0
                    + (31.0 / 180.0) * self.ellipsoid_e_squared.powi(2)
                    + (517.0 / 5040.0) * self.ellipsoid_e_squared.powi(3))
                    * (beta_ * 2.0).sin()
                    + ((23.0 / 360.0) * self.ellipsoid_e_squared.powi(2)
                        + (251.0 / 3780.0) * self.ellipsoid_e_squared.powi(3))
                        * (beta_ * 4.0).sin()
                    + (761.0 / 45360.0) * self.ellipsoid_e_squared.powi(3) * (beta_ + 6.0).sin()),
        )
    }
}

impl PseudoSerialize for LambertAzimuthalEqualAreaProjection {
    fn to_constructed(&self) -> String {
        format!(
            r"LambertAzimuthalEqualAreaProjection{{
    lon_orig: {}f64,
    false_e: {}f64,
    false_n: {}f64,
    ellipsoid_e: {}f64,
    ellipsoid_e_squared: {}f64,

    q_P: {}f64,
    beta_O: {}f64,
    R_q: {}f64,
    D: {}f64,
}}",
            self.lon_orig,
            self.false_e,
            self.false_n,
            self.ellipsoid_e,
            self.ellipsoid_e_squared,
            self.q_P,
            self.beta_O,
            self.R_q,
            self.D
        )
    }
}

impl DbContstruct for LambertAzimuthalEqualAreaProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        /*
        ImplementedProjection::new(
            9820,
            &[8802, 8801, 8806, 8807],
            "LambertAzimuthalEqualAreaParams",
            "LambertAzimuthalEqualAreaProjection"
        )
        */
        let params = LambertAzimuthalEqualAreaParams::new(
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
    LambertAzimuthalEqualAreaProjection::from_database_params(params, &ell).to_constructed()
}

impl GetterContstruct for LambertAzimuthalEqualAreaProjection {
    fn with_db_getter<G>(mut getter: G, ellipsoid: &Ellipsoid) -> Option<Self>
    where
        G: FnMut(u32) -> Option<f64>,
    {
        let params = LambertAzimuthalEqualAreaParams::new(
            getter(8802)?,
            getter(8801)?,
            getter(8806)?,
            getter(8807)?,
        );
        Some(Self::new(ellipsoid, &params))
    }
}

#[cfg(test)]
mod tests {

    use crate::ellipsoid::Ellipsoid;
    use crate::lambert_azimuthal_equal_area::*;
    use crate::traits::*;

    #[test]
    fn lambert_azimuthal_equal_area_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378137.0, 298.2572221);
        let params = LambertAzimuthalEqualAreaParams::new(
            10.0f64.to_radians(),
            52.0f64.to_radians(),
            4_321_000.0,
            3_210_000.0,
        );

        let projection = LambertAzimuthalEqualAreaProjection::new(&ell, &params);
        let easting_goal = 3962799.45;
        let northing_goal = 2999718.85;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        let (easting, northing) = projection.deg_to_projected(lon, lat);
        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.01);

        assert!((northing - northing_goal).abs() < 0.05);
    }
}
