//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use crate::ellipsoid_constructor;

use crate::transverse_mercator::{TransverseMercatorConversion, TransverseMercatorParams};
use crate::lambert_azimuthal_equal_area::{LambertAzimuthalEqualAreaConversion, LambertAzimuthalEqualAreaParams};
use crate::zero_transformation::ZeroTransformation;

use crate::traits::CoordTransform;

include!(concat!(env!("OUT_DIR"), "/projection_constructors.rs"));
