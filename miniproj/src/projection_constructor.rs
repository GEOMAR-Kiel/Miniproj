//This file is licensed under EUPL v1.2

use miniproj_ops::albers_equal_area::AlbersEqualAreaProjection;
use miniproj_ops::identity_projection::IdentityProjection;
use miniproj_ops::lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaProjection;
use miniproj_ops::lambert_conic_conformal::{
    LambertConic1SPAProjection, LambertConic2SPProjection,
};
use miniproj_ops::popvis_pseudo_mercator::PopVisPseudoMercatorProjection;
use miniproj_ops::stereographic::{ObliqueStereographicProjection, PolarStereographicAProjection};
use miniproj_ops::transverse_mercator::TransverseMercatorProjection;
use miniproj_ops::{Projection, Ellipsoid};

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));

/// Returns the Projected Coordinate Reference System corresponding to the EPSG code passed as the argument.
/// If the code refers to a projection that is not implemented, the method returns `None`
pub fn get_projection(code: u32) -> Option<&'static dyn Projection> {
    PROJECTIONS.get(&code).cloned()
}

/// Returns the EPSG code of the ellipsoid that is associated with the projection
/// corresponding to `projection_code`. Returns `None` if the projection is
/// unknown.
pub fn get_ellipsoid_code(projection_code: u32) -> Option<u32> {
    ELLIPSOIDS.get(&projection_code).copied()
}

/// Create the Projection corresponding to the EPSG code passed as the argument, using the passed ellipsoid.
/// The `&Ellipsoid` is not held by the returned projection, if you want the projection for a different
/// ellipsoid you need to construct it again.
pub fn create_projection(code: u32, ellipsoid: &Ellipsoid) -> Option<Box<dyn Projection>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_projection_is_send_sync() {
        fn is_send_sync<T: Send + Sync>(_: T) {}

        is_send_sync(get_projection(4326));
    }
}
