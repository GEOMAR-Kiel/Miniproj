//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer


//! This crate implements geographic transformations between different coordinate systems defined
//! by the European Petroleum Survey Group.
//! 
//! Think of it as a very lightweight [PROJ](https://github.com/OSGeo/PROJ)
//! 
//! Currently, only the transverse mercator, stereographic and lamber azimuthal equal area coordinate systems are defined.
//! 
//! It was written at the [GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of the [Digital Earth Project](https://www.digitalearth-hgf.de/).
//! 
//! As many of the other components created in this project, it is licensed under [EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
//! 
//! Usage example:
//! ```rust
//! //Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
//! use epsg_geodetic_parameters::{get_coord_transform, CoordTransform};
//! let converter = get_coord_transform(32632).expect("Coordinate conversion not implemented");
//! 
//! //Coordinates of the office where this converter was written in UTM:
//! let (x,y) = (576935.86f64, 6020593.46f64);
//! 
//! //To get the latitude and longitude, use the CoordTransform::to_deg method.
//! let (lon, lat) = converter.to_deg(x,y);
//!
//! assert!((lon - 10.183034) < 0.000001);
//! assert!((lat - 54.327389) < 0.000001);
//! ```

#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_float_classify)]
#![feature(const_float_bits_conv)]
/*
pub const fn to_radians_ext(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}
pub const fn to_degrees_ext(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}*/

mod transverse_mercator;
mod stereographic;
mod lambert_azimuthal_equal_area;
mod zero_transformation;

mod ellipsoid;

mod ellipsoid_constructor;
mod projection_constructor;


use projection_constructor as projections;

mod params;

mod traits;
pub use traits::CoordTransform;

pub use projection_constructor::get_coord_transform;

struct BoxedTransform{
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