//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::ellipsoid::{Ellipsoid};

/// Two-dimensional coordinate operation
pub trait Projection: Send + Sync{
    ///Converts from a coordinate in the target coordinate system to lon/lat in EPSG 4326 in radians
    fn to_rad(&self, x: f64, y: f64) -> (f64, f64);
    ///Converts from a coordinate in radians in EPSG 4326 to the target coordinate system
    fn from_rad(&self, lon: f64, lat: f64) -> (f64, f64);
    ///Converts from a coordinate in the target coordinate system to lon/lat in EPSG 4326 in degrees
    fn to_deg(&self, x: f64, y: f64) -> (f64, f64) {
        let tmp = self.to_rad(x, y);
        (tmp.0.to_degrees(), tmp.1.to_degrees())
    }
    ///Converts from a coordinate in degrees in EPSG 4326 to the target coordinate system
    fn from_deg(&self, lon: f64, lat: f64) -> (f64, f64) {
        self.from_rad(lon.to_radians(), lat.to_radians())
    }
}

pub trait PseudoSerialize {
    fn to_constructed(&self) -> String;
}

pub trait DbContstruct {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self;
}