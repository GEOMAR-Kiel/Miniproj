//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::traits::CoordTransform;

/// Parameterless conversion that is a no-op in degrees and otherwise converts between degrees and radians.
pub struct ZeroTransformation;

impl CoordTransform for ZeroTransformation{
    fn to_rad(&self, x: f64, y: f64) -> (f64, f64) {
        (x.to_radians(),y.to_radians())
    }

    fn from_rad(&self, lon: f64, lat: f64) -> (f64, f64) {
        (lon.to_degrees(), lat.to_degrees())
    }

    fn to_deg(&self, x: f64, y: f64) -> (f64, f64) {
        (x,y)
    }

    fn from_deg(&self, lon: f64, lat: f64) -> (f64, f64) {
        (lon, lat)
    }
}
