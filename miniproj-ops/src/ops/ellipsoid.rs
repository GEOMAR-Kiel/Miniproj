//This file is licensed under EUPL v1.2


/// Ellipsoid, a simple approximation of the earth's shape used in most `Projection`s
#[derive(Copy, Clone, Debug)]
pub struct Ellipsoid {
    /// semi-major axis
    a: f64,
    // /// semi-minor axis
    b: f64,
    /// inverse flattening
    f_inv: f64,
    /// flattening
    f: f64,
    /// eccentricity
    e: f64,
    /// eccentricity squared
    e_squared: f64
}
impl Ellipsoid {

    /// Construct an ellipsoid from major and minor half axis.
    pub fn from_a_b(a: f64, b: f64) -> Self {
        let f = (a - b) / a;
        let e_squared = (2f64 * f) - f.powi(2);
        Self{
            a,
            b,
            f,
            f_inv: a / (a - b),
            e_squared,
            e: e_squared.sqrt() 
        }
    }

    /// Construct an ellipsoid from major half axis and inverse flattening.
    pub fn from_a_f_inv(a: f64, f_inv: f64) -> Self {
        let f = 1.0 / f_inv;
        let e_squared = (2f64 / f_inv) - f_inv.powi(-2);
        Self{
            a,
            b: a - a / f_inv,
            f,
            f_inv,
            e_squared,
            e: e_squared.sqrt() 
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
        self.e
    }

    /// Get eccentricity squared.
    pub fn e_squared(&self) -> f64 {
        self.e_squared
    }

    /// Get secondary eccentricity.
    pub fn e_2(&self) -> f64 {
        f64::sqrt(self.e_squared() / (1.0 - self.e_squared()))
    }

    /// Get radius of curvature in the meridian, latitude in radians.
    pub fn rho(&self, lat: f64) -> f64 {
        self.a * (1.0 - self.e_squared()) / (1.0 - self.e_squared() * lat.sin().powi(2)).powf(1.5)
    }

    /// Get radius of curvature in the prime vertical, latitude in radians.
    pub fn ny(&self, lat: f64) -> f64 {
        self.a / (1.0 - self.e_squared() * lat.sin().powi(2)).sqrt()
    }

    /// Get radius of authalic sphere (sphere with the same surface area as the ellipsoid).
    pub fn rad_auth(&self) -> f64 {
        self.a * ((1.0 - ((1.0 - self.e_squared()) / (2.0 * self.e())) * f64::ln((1.0 - self.e()) / (1.0 + self.e()))) * 0.5 ).sqrt()
    }

    /// Get radius of conformal sphere.
    pub fn rad_conformal(&self, lat: f64) -> f64 {
        f64::sqrt(self.rho(lat) * self.ny(lat))
    }

    /// Convert from geocentric position in meters to `(longitude, latitude, height)`, geographic position in decimal degrees and *ellipsoid* height in meters.
    pub fn degrees_from_geocentric(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let (lon, lat, h) = self.radians_from_geocentric(x, y, z);
        (lon.to_degrees(), lat.to_degrees(), h)
    }

    /// Convert from geographic position in degrees and *ellipsoid* height in meters to `(x, y, z)`, geocentric position in meters.
    pub fn degrees_to_geocentric(&self, lon: f64, lat: f64, height: f64) -> (f64, f64, f64) {
        self.radians_to_geocentric(lon.to_radians(), lat.to_radians(), height)
    }

    /// Convert from geocentric position in meters to `(longitude, latitude, height)`, geographic position in radians and *ellipsoid* height in meters.
    pub fn radians_from_geocentric(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let lon = y.atan2(x);
        let epsilon = self.e_squared() / (1f64 - self.e_squared());
        let p = (x.powi(2) + y.powi(2)).sqrt();
        let q = (z * self.a).atan2(p * self.b);
        let lat = (z + epsilon * self.b * q.sin().powi(3)).atan2(p - self.e_squared() * self.a * q.cos().powi(3));
        let h = (p / lat.cos()) - self.ny(lat);
        (lon, lat, h)
    }

    /// Convert from geographic position in radians and *ellipsoid* height in meters to `(x, y, z)`, geocentric position in meters.
    pub fn radians_to_geocentric(&self, lon: f64, lat: f64, height: f64) -> (f64, f64, f64) {
        let ny = self.ny(lat);
        let r = self.ny(lat) + height;
        (
            r * lat.cos() * lon.cos(),
            r * lat.cos() * lon.sin(),
            (1f64 - self.e_squared() * ny + height) * lat.sin()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Ellipsoid;


    #[test]
    fn geocentric_roundtrip() {
        let ell = Ellipsoid::from_a_f_inv(6378137.000, 298.2572236);
        let expected_lat = 53f64 + 48f64 / 60f64 + 33.820 / 3600f64;
        let expected_lon = 2f64 + 7f64 / 60f64 + 46.380 / 3600f64;
        let expected_eh = 73.0;

        let (lon, lat, eh) = ell.degrees_from_geocentric(
            3771793.968, 
            140253.342,
            5124304.349
        );
        eprintln!("lon: {expected_lon} - {lon}");
        eprintln!("lat: {expected_lat} - {lat}");
        eprintln!("lon: {expected_eh} - {eh}");
        assert!((expected_lon - lon).abs() < 0.01 / 3600.0);
        assert!((expected_lat - lat).abs() < 0.01 / 3600.0);
        assert!((expected_eh - eh).abs() < 0.1);
    }

}