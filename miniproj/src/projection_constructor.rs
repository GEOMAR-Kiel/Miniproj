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
use miniproj_ops::{CoordOperation, Projection, ProjectionParams, Ellipsoid};

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));

/// Returns the Coordinate Reference System corresponding to the EPSG code passed as the argument.
/// If the code refers to a projection that is not implemented, the method returns `None`
pub fn get_projection(code: u32) -> Option<&'static dyn Projection> {
    let params = PROJECTION_PARAMS.get(&code)?;
    todo!()
}

/// Returns the EPSG code of the ellipsoid that is associated with the projection
/// corresponding to `projection_code`. Returns `None` if the projection is
/// unknown.
pub fn get_ellipsoid_code(projection_code: u32) -> Option<u32> {
    PROJECTION_PARAMS.get(&projection_code).map(|(_, e)| e).copied()
}

/// Returns the Name of the Coordinate Reference System. This is a temporary method that will be removed.
#[deprecated]
pub fn get_reference_system_name(code: u32) -> Option<&'static str> {
    NAMES.get(&code).copied()
}

/// Returns one or multiple geographic areas that the reference system applies to.
/// Values are in `[east, north, west, south]`` order. This is a temporary method that will be removed.
#[deprecated]
pub fn get_reference_system_areas(code: u32) -> Option<&'static [[f64; 4]]> {
    AREAS.get(&code).filter(|a| !a.is_empty()).copied()
}

#[deprecated]
pub fn all_names() -> impl Iterator<Item = (u32, &'static str)> {
    NAMES.entries().map(|(c, n)| (*c, *n))
}

pub fn get_transformation<F, T>(from: u32, to: u32) -> Option<Box<dyn CoordOperation<F, T>>> {
    None
}

pub fn get_transformation_at<F, T>(
    from: u32,
    from_epoch: f32,
    to: u32,
    to_epoch: f32,
) -> Option<Box<dyn CoordOperation<F, T>>> {
    None
}

/// Create the Projection corresponding to the EPSG code passed as the argument, using the passed ellipsoid.
/// The `&Ellipsoid` is not held by the returned projection, if you want the projection for a different
/// ellipsoid you need to construct it again.
//pub fn create_projection(code: u32, ellipsoid: &Ellipsoid) -> Option<Box<dyn Projection>> {
//    todo!()
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_projection_is_send_sync() {
        fn is_send_sync<T: Send + Sync>(_: T) {}

        is_send_sync(get_projection(4326));
    }
}
