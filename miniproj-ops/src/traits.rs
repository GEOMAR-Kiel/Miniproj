//This file is licensed under EUPL v1.2

use crate::ellipsoid::Ellipsoid;

/// Two-dimensional coordinate operation
pub trait Projection: Send + Sync {
    ///Converts from a coordinate in the target coordinate system to lon/lat in EPSG 4326 in radians
    fn projected_to_rad(&self, x: f64, y: f64) -> (f64, f64);

    ///Converts from a geographic coordinate in radians to a projected coordinate `(x, y)`, usually in meters.
    fn rad_to_projected(&self, lon: f64, lat: f64) -> (f64, f64);

    ///Converts from projected coordinates to geographic coordinates `(longitude, latitude)` in decimal degrees.
    fn projected_to_deg(&self, x: f64, y: f64) -> (f64, f64) {
        let tmp = self.projected_to_rad(x, y);
        (tmp.0.to_degrees(), tmp.1.to_degrees())
    }

    ///Converts from a geographic coordinate in degrees to a projected coordinate `(x, y)`, usually in meters.
    fn deg_to_projected(&self, lon: f64, lat: f64) -> (f64, f64) {
        self.rad_to_projected(lon.to_radians(), lat.to_radians())
    }
}

pub trait PseudoSerialize {
    fn to_constructed(&self) -> String;
}

pub trait DbContstruct {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self;
}
