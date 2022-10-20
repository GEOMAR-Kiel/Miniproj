//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_float_classify)]
#![feature(const_float_bits_conv)]

pub const fn to_radians_ext(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}
pub const fn to_degrees_ext(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

pub mod transverse_mercator;
pub mod stereographic;
pub mod lambert_azimuthal_equal_area;
pub mod zero_transformation;

pub mod ellipsoid;

pub mod ellipsoid_constructor;
pub mod projection_constructor;

pub use ellipsoid_constructor as ellipsoids;
pub use projection_constructor as projections;

pub mod traits;
pub use traits::CoordTransform;

pub struct BoxedTransform{
    transform: Box<dyn CoordTransform>,
    epsg_code: u32
}

impl BoxedTransform{
    pub fn from_epsg_code(epsg_code: u32) -> Self{
        BoxedTransform { transform: crate::projections::get_coord_transform(epsg_code).unwrap(), epsg_code }
    }
}

impl CoordTransform for BoxedTransform{
    fn to_rad(&self, x: f64, y: f64) -> (f64, f64) {
        self.transform.to_rad(x, y)
    }

    fn from_rad(&self, lon: f64, lat: f64) -> (f64, f64) {
       self.transform.from_rad(lon, lat)
    }
}

impl Clone for BoxedTransform{
    fn clone(&self) -> Self {
        Self::from_epsg_code(self.epsg_code)
    }
}

impl std::fmt::Debug for BoxedTransform{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedTransform").field("transform", &"[dyn CoordTransform instance]").field("epsg_code", &self.epsg_code).finish()
    }
}