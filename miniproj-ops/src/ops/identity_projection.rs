//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::traits::Projection;

/// Parameterless projection that is a no-op in degrees and otherwise converts between degrees and radians.
pub struct IdentityProjection;

impl Projection for IdentityProjection {
    fn projected_to_rad(&self, x: f64, y: f64) -> (f64, f64) {
        (x.to_radians(), y.to_radians())
    }

    fn rad_to_projected(&self, lon: f64, lat: f64) -> (f64, f64) {
        (lon.to_degrees(), lat.to_degrees())
    }

    fn projected_to_deg(&self, x: f64, y: f64) -> (f64, f64) {
        (x, y)
    }

    fn deg_to_projected(&self, lon: f64, lat: f64) -> (f64, f64) {
        (lon, lat)
    }
}
