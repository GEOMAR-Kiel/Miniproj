//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use miniproj_ops::CoordTransform;
use miniproj_ops::transverse_mercator::TransverseMercatorConversion;
use miniproj_ops::lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaConversion;
use miniproj_ops::zero_transformation::ZeroTransformation;

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));

/// Returns the CoordTransform corresponding to the EPSG code passed as the argument.
/// If the code refers to a projection that is not implemented, the method returns None
pub fn get_coord_transform(code: u32) -> Option<&'static (dyn CoordTransform + Send + Sync)> {
    TRANSFORMS.get(&code).cloned()
}