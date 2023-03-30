//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use epsg_coordoperations::CoordTransform;
use epsg_coordoperations::transverse_mercator::TransverseMercatorConversion;
use epsg_coordoperations::lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaConversion;

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));

///Creates a boxed CoordTransform object for a projection specified through a certain EPSG code.
///If the code refers to a projection that is not implemented, the method returns None
pub fn get_coord_transform(code: u32) -> Option<&'static (dyn CoordTransform + Send + Sync)> {
    TRANSFORMS.get(&code).cloned()
}