//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use miniproj_ops::Projection;
use miniproj_ops::transverse_mercator::TransverseMercatorProjection;
use miniproj_ops::lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaProjection;
use miniproj_ops::zero_projection::ZeroProjection;
use miniproj_ops::stereographic::PolarStereographicAProjection;
use miniproj_ops::lambert_conic_conformal::{LambertConic2SPProjection, LambertConic1SPAProjection};
use miniproj_ops::popvis_pseudo_mercator::PopVisPseudoMercatorProjection;

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));

/// Returns the Projection corresponding to the EPSG code passed as the argument.
/// If the code refers to a projection that is not implemented, the method returns `None`
pub fn get_projection(code: u32) -> Option<&'static (dyn Projection + Send + Sync)> {
    PROJECTIONS.get(&code).cloned()
}

/// Returns the EPSG code of the ellipsoid that is associated with the projection
/// corresponding to `projection_code`. Returns `None` if the projection is
/// unknown.
pub fn get_ellipsoid_code(projection_code: u32) -> Option<u32> {
    ELLIPSOIDS.get(&projection_code).copied()
}
