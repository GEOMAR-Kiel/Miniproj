//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

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
    false_n: f64,
}

impl PolarStereographicAParams {
    pub const fn new(
        lon_orig: f64,
        lat_orig: f64,
        k_orig: f64,
        false_e: f64,
        false_n: f64,
    ) -> Self {
        Self {
            lat_orig,
            lon_orig,
            k_orig,
            false_e,
            false_n,
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
pub struct PolarStereographicAProjection {
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
    pub ell_e: f64,
}

impl PolarStereographicAProjection {
    pub fn new(ell: &Ellipsoid, params: &PolarStereographicAParams) -> Self {
        let t_rho_factor =
            ((1.0 + ell.e()).powf(1.0 + ell.e()) * (1.0 - ell.e()).powf(1.0 - ell.e())).sqrt()
                / (2.0 * ell.a() * params.k_orig());
        let phi_2_chi_sin_summand_factor = ell.e_squared() / 2.0
            + 5.0 * ell.e_squared().powi(2) / 24.0
            + ell.e_squared().powi(3) / 12.0
            + 13.0 * ell.e_squared().powi(4) / 360.0;
        let phi_4_chi_sin_summand_factor = 7.0 * ell.e_squared().powi(2) / 48.0
            + 29.0 * ell.e_squared().powi(3) / 240.0
            + ell.e_squared().powi(4) / 11520.0;
        let phi_6_chi_sin_summand_factor =
            7.0 * ell.e_squared().powi(3) / 120.0 + 81.0 * ell.e_squared().powi(4) / 1120.0;
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

impl crate::traits::Projection for PolarStereographicAProjection {
    fn rad_to_projected(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        if self.lat_orig < 0.0 {
            // North Pole Case
            let t = f64::tan(std::f64::consts::FRAC_PI_4 - latitude / 2.0)
                * ((1.0 + self.ell_e * latitude.sin()) / (1.0 - self.ell_e * latitude.sin()))
                    .powf(self.ell_e / 2.0);
            let rho = t / self.t_rho_factor;
            (
                self.false_e + rho * f64::sin(longitude - self.lon_orig),
                self.false_n - rho * f64::cos(longitude - self.lon_orig),
            )
        } else {
            // South Pole Case
            let t = f64::tan(std::f64::consts::FRAC_PI_4 + latitude / 2.0)
                / ((1.0 + self.ell_e * latitude.sin()) / (1.0 - self.ell_e * latitude.sin()))
                    .powf(self.ell_e / 2.0);
            let rho = t / self.t_rho_factor;
            (
                self.false_e + rho * f64::sin(longitude - self.lon_orig),
                self.false_n + rho * f64::cos(longitude - self.lon_orig),
            )
        }
    }

    fn projected_to_rad(&self, easting: f64, northing: f64) -> (f64, f64) {
        let rho_ = ((easting - self.false_e).powi(2) + (northing - self.false_n).powi(2)).sqrt();
        let t_ = rho_ * self.t_rho_factor;
        let chi = if self.lat_orig < 0.0 {
            // North Pole Case
            FRAC_PI_2 - 2.0 * t_.atan()
        } else {
            // South Pole Case
            2.0 * t_.atan() - FRAC_PI_2
        };
        let phi = chi
            + self.phi_2_chi_sin_summand_factor * (2.0 * chi).sin()
            + self.phi_4_chi_sin_summand_factor * (4.0 * chi).sin()
            + self.phi_6_chi_sin_summand_factor * (6.0 * chi).sin()
            + self.phi_8_chi_sin_summand_factor * (8.0 * chi).sin();
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
impl DbContstruct for PolarStereographicAProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = PolarStereographicAParams::new(
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
impl PseudoSerialize for PolarStereographicAProjection {
    fn to_constructed(&self) -> String {
        format!(
            r"PolarStereographicAProjection{{
    t_rho_factor: {}f64,
    phi_2_chi_sin_summand_factor: {}f64,
    phi_4_chi_sin_summand_factor: {}f64,
    phi_6_chi_sin_summand_factor: {}f64,
    phi_8_chi_sin_summand_factor: {}f64,
    
    lat_orig: {}f64,
    lon_orig: {}f64,
    false_e: {}f64,
    false_n: {}f64,

    ell_e: {}f64
}}",
            self.t_rho_factor,
            self.phi_2_chi_sin_summand_factor,
            self.phi_4_chi_sin_summand_factor,
            self.phi_6_chi_sin_summand_factor,
            self.phi_8_chi_sin_summand_factor,
            self.lat_orig,
            self.lon_orig,
            self.false_e,
            self.false_n,
            self.ell_e
        )
    }
}
pub fn direct_projection_a(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    PolarStereographicAProjection::from_database_params(params, &ell).to_constructed()
}

#[derive(Copy, Clone, Debug)]
pub struct ObliqueStereographicParams {
    // Longitude of natural origin
    lon_orig: f64,
    // Latitude of natural origin
    lat_orig: f64,
    // Scale factor at natural origin
    k_orig: f64,
    // False easting
    false_e: f64,
    // False northing
    false_n: f64,
}

impl ObliqueStereographicParams {
    pub fn new(lon_orig: f64, lat_orig: f64, k_orig: f64, false_e: f64, false_n: f64) -> Self {
        assert!(lat_orig > 0f64);
        Self {
            lat_orig,
            lon_orig,
            k_orig,
            false_e,
            false_n,
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
pub struct ObliqueStereographicProjection {
    pub false_e: f64,
    pub false_n: f64,
    pub chi_O: f64,
    pub R_k_O_2: f64,
    pub c: f64,
    pub ellipsoid_e: f64,
    pub ellipsoid_e_sq: f64,
    pub n: f64,
    pub lon_orig: f64,
    pub g: f64,
    pub h: f64,
}

impl ObliqueStereographicProjection {
    const MAX_ITERATIONS: usize = 4;

    #[allow(non_snake_case)]
    pub fn new(ell: &Ellipsoid, params: &ObliqueStereographicParams) -> Self {
        let rho_O = ell.rho(params.lat_orig());
        let ny_O = ell.ny(params.lat_orig());
        let R = (rho_O * ny_O).sqrt();
        let n = (1f64
            + ((ell.e_squared() * params.lat_orig().cos().powi(4)) / (1f64 - ell.e_squared())))
        .sqrt();
        let S_1 = (1f64 + params.lat_orig().sin()) / (1f64 - params.lat_orig.sin());
        let S_2 =
            (1f64 - ell.e() * params.lat_orig().sin()) / (1f64 + ell.e() * params.lat_orig().sin());
        let w_1 = (S_1 * S_2.powf(ell.e())).powf(n);
        let chi_OO_sin = (w_1 - 1f64) / (w_1 + 1f64);
        let c = (n + params.lat_orig().sin()) * (1f64 - chi_OO_sin)
            / ((n - params.lat_orig().sin()) * (1f64 + chi_OO_sin));
        let w_2 = c * w_1;
        let chi_O = ((w_2 - 1f64) / (w_2 + 1f64)).asin();
        let g = 2f64 * R * params.k_orig() * (FRAC_PI_4 - chi_O / 2f64).tan();
        let h = 4f64 * R * params.k_orig() * chi_O.tan() + g;
        Self {
            false_e: params.false_e(),
            false_n: params.false_n(),
            chi_O,
            R_k_O_2: R * params.k_orig() * 2f64,
            c,
            ellipsoid_e: ell.e(),
            ellipsoid_e_sq: ell.e_squared(),
            n,
            lon_orig: params.lon_orig(),
            g,
            h,
        }
    }
}

impl crate::traits::Projection for ObliqueStereographicProjection {
    #[allow(non_snake_case)]
    fn projected_to_rad(&self, x: f64, y: f64) -> (f64, f64) {
        let i = (x - self.false_e).atan2(self.h + (y - self.false_n));
        let j = (x - self.false_e).atan2(self.g - (y - self.false_n)) - i;
        let chi = self.chi_O
            + 2f64
                * (((y - self.false_n) - (x - self.false_e) * (j / 2f64).tan()) / self.R_k_O_2)
                    .atan();
        let psi = 0.5 * ((1f64 + chi.sin()) / (self.c * (1f64 - chi.sin()))).ln() / self.n;
        let mut phi = 2f64 * psi.exp().atan() - FRAC_PI_2;
        for _ in 0..Self::MAX_ITERATIONS {
            let psi_ = ((phi / 2f64 + FRAC_PI_4).tan()
                * ((1f64 - self.ellipsoid_e * phi.sin()) / (1f64 + self.ellipsoid_e * phi.sin()))
                    .powf(self.ellipsoid_e / 2f64))
            .ln();
            phi = phi
                - (psi_ - psi) * phi.cos() * (1f64 - self.ellipsoid_e_sq * phi.sin().powi(2))
                    / (1f64 - self.ellipsoid_e_sq);
        }
        let DeltaLambda = j + 2f64 * i;
        (DeltaLambda / self.n + self.lon_orig, phi)
    }

    #[allow(non_snake_case)]
    fn rad_to_projected(&self, lon: f64, lat: f64) -> (f64, f64) {
        let S_a = (1f64 + lat.sin()) / (1f64 - lat.sin());
        let S_b = (1f64 - self.ellipsoid_e * lat.sin()) / (1f64 + self.ellipsoid_e * lat.sin());
        let DeltaLambda = self.n * (lon - self.lon_orig);
        let w = self.c * (S_a * S_b.powf(self.ellipsoid_e)).powf(self.n);
        let chi = ((w - 1f64) / (w + 1f64)).asin();
        let B =
            1f64 + chi.sin() * self.chi_O.sin() + chi.cos() * self.chi_O.cos() * DeltaLambda.cos();
        (
            self.false_e + self.R_k_O_2 * chi.cos() * DeltaLambda.sin() / B,
            self.false_n
                + self.R_k_O_2
                    * (chi.sin() * self.chi_O.cos()
                        - chi.cos() * self.chi_O.sin() * DeltaLambda.cos())
                    / B,
        )
    }
}

impl DbContstruct for ObliqueStereographicProjection {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self {
        let params = ObliqueStereographicParams::new(
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

impl PseudoSerialize for ObliqueStereographicProjection {
    fn to_constructed(&self) -> String {
        format!(
            r"ObliqueStereographicProjection{{
    false_e: {}f64,
    false_n: {}f64,
    chi_O: {}f64,
    R_k_O_2: {}f64,
    c: {}f64,
    ellipsoid_e: {}f64,
    ellipsoid_e_sq: {}f64,
    n: {}f64,
    lon_orig: {}f64,
    g: {}f64,
    h: {}f64
}}",
            self.false_e,
            self.false_n,
            self.chi_O,
            self.R_k_O_2,
            self.c,
            self.ellipsoid_e,
            self.ellipsoid_e_sq,
            self.n,
            self.lon_orig,
            self.g,
            self.h
        )
    }
}

pub fn direct_projection_oblique(params: &[(u32, f64)], ell: Ellipsoid) -> String {
    ObliqueStereographicProjection::from_database_params(params, &ell).to_constructed()
}
#[cfg(test)]
mod tests {

    use crate::ellipsoid::Ellipsoid;
    use crate::stereographic::*;
    use crate::traits::*;

    #[test]
    fn polar_stereographic_a_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6378137.0, 298.257223563);
        let params = PolarStereographicAParams::new(
            0.0f64.to_radians(),
            -90.0f64.to_radians(),
            0.994,
            2_000_000.0,
            2_000_000.0,
        );

        let projection = PolarStereographicAProjection::new(&ell, &params);
        let easting_goal = 3329416.75;
        let northing_goal = 632668.43;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        let (easting, northing) = projection.deg_to_projected(lon, lat);

        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.01);

        assert!((northing - northing_goal).abs() < 0.01);
    }

    #[test]
    fn oblique_stereographic_consistency() {
        let ell = Ellipsoid::from_a_f_inv(6377397.155, 299.15281);
        let params = ObliqueStereographicParams::new(
            0.094032038,
            0.910296727,
            0.9999079,
            155000.0,
            463000.0,
        );

        let projection = ObliqueStereographicProjection::new(&ell, &params);
        let easting_goal = 196105.283;
        let northing_goal = 557057.739;
        let (lon, lat) = projection.projected_to_deg(easting_goal, northing_goal);
        eprintln!("lon: {lon}, lat: {lat}");
        let (easting, northing) = projection.deg_to_projected(lon, lat);

        eprintln!("easting: {easting_goal} - {easting}");
        eprintln!("northing: {northing_goal} - {northing}");

        assert!((easting - easting_goal).abs() < 0.01);

        assert!((northing - northing_goal).abs() < 0.01);
    }
}
