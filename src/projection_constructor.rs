//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::ellipsoid_constructor;

use crate::transverse_mercator::{TransverseMercatorParams};
use crate::lambert_azimuthal_equal_area::{LambertAzimuthalEqualAreaParams};
use crate::zero_transformation::ZeroTransformation;

use crate::traits::CoordTransform;
use crate::params::Params;

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));

///Creates a boxed CoordTransform object for a projection specified through a certain EPSG code.
///If the code refers to a projection that is not implemented, the method returns None
pub fn get_coord_transform(code: u32) -> Option<Box<dyn CoordTransform>> {
    if code == 4326{
        Some(Box::new(ZeroTransformation) as Box<dyn CoordTransform>)
    }else{
        ellipsoid_constructor::get_ellipsoid(*ELLIPSOID_CODES.get(&code).unwrap())
        .and_then(|e|PARAMETERS.get(&code).map(|p| (e, p)))
        .and_then(|(e,p)| NAMES.get(&code).map(|n| (e,p,n)))
        .and_then(|(e,p,n)| CONV_TYPES.get(&code).map(|c| (e,p,n,c)))
        .map(|(e,p,_n,c)| p.apply_named_conversion(e, c))
    }
}