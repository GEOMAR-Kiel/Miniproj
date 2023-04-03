//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer


/// Ellipsoid, a simple approximation of the earth's shape used in most `CoordTransform`s
#[derive(Copy, Clone, Debug)]
pub struct Ellipsoid {
    /// semi-major axis
    a: f64,
    /// semi-minor axis
    b: f64,
    /// inverse flattening
    f_inv: f64,
    /// flattening
    f: f64,

}
impl Ellipsoid {

    /// Construct an ellipsoid from major and minor half axis.
    pub fn from_a_b(a: f64, b: f64) -> Self {
        let f = (a - b) / a;
        Self{
            a,
            b,
            f,
            f_inv: a / (a - b),
        }
    }

    /// Construct an ellipsoid from major half axis and inverse flattening.
    pub fn from_a_f_inv(a: f64, f_inv: f64) -> Self {
        let f = 1.0 / f_inv;
        Self{
            a,
            b: a - a / f_inv,
            f,
            f_inv,
        }
    }

    /// Get major half axis.
    pub fn a(&self) -> f64 {
        self.a
    }

    /// Get minor half axis.
    pub fn b(&self) -> f64 {
        self.b
    }

    /// Get inverse flattening.
    pub fn f_inv(&self) -> f64 {
        self.f_inv
    }

    /// Get flattening.
    pub fn f(&self) -> f64 {
        self.f
    }

    /// Get eccentricity.
    pub fn e(&self) -> f64 {
        f64::sqrt(2.0 * self.f() - self.f().powi(2))
    }

    /// Get secondary eccentricity.
    pub fn e_2(&self) -> f64 {
        f64::sqrt(self.e().powi(2) / (1.0 - self.e().powi(2)))
    }

    /// Get radius of curvature in the meridian, latitude in radians.
    pub fn rho(&self, lat: f64) -> f64 {
        self.a * (1.0 - self.e().powi(2)) / (1.0 - self.e().powi(2) * lat.sin().powi(2)).powf(1.5)
    }

    /// Get radius of curvature in the prime vertical, latitude in radians.
    pub fn ny(&self, lat: f64) -> f64 {
        self.a / (1.0 - self.e().powi(2) * lat.sin().powi(2)).sqrt()
    }

    /// Get radius of authalic sphere (sphere with the same surface area as the ellipsoid).
    pub fn rad_auth(&self) -> f64 {
        self.a * ((1.0 - ((1.0 - self.e().powi(2)) / (2.0 * self.e())) * f64::ln((1.0 - self.e()) / (1.0 + self.e()))) * 0.5 ).sqrt()
    }

    /// Get radius of conformal sphere.
    pub fn rad_conformal(&self, lat: f64) -> f64 {
        f64::sqrt(self.rho(lat) * self.ny(lat))
    }
}