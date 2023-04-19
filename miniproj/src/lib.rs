//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer


//! This crate implements geodetic operations defined by the [European Petroleum Survey Group](https://epsg.org/home.html).
//!
//! Think of it as a very lightweight [PROJ](https://github.com/OSGeo/PROJ).
//!
//!Currently, only the transverse mercator and lambert azimuthal equal area coordinate operations are completely implemented.
//!
//! It was written at the [GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of the [Digital Earth Project](https://www.digitalearth-hgf.de/).
//!
//! As many of the other components created in this project, it is licensed under [EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12). This license
//! does not apply to the projections themselves. The database files are extracts from the EPSG
//! Geodetic Parameter Registry and distributed under [their own Terms of Use](epsg-geodetic-parameter-gen/data/terms.md).
//!
//! Usage example:
//! ```rust
//! // Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
//! use miniproj::{get_projection, Projection};
//! let converter = get_projection(32632).expect("Projection not implemented");
//! 
//! // Coordinates of the office where this converter was written in UTM:
//! let (x,y) = (576935.86f64, 6020593.46f64);
//! 
//! // To get the latitude and longitude, use the Projection::to_deg method.
//! let (lon, lat) = converter.to_deg(x,y);
//!
//! assert!((lon - 10.183034) < 0.000001);
//! assert!((lat - 54.327389) < 0.000001);
//! ```


#![feature(const_float_bits_conv)]
/*
pub const fn to_radians_ext(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}
pub const fn to_degrees_ext(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}*/

mod projection_constructor;

#[doc(inline)]
pub use miniproj_ops::Projection;
#[doc(inline)]
pub use projection_constructor::get_projection;

struct BoxedTransform{
    transform: Box<dyn Projection>,
    epsg_code: u32
}

impl Projection for BoxedTransform{
    fn to_rad(&self, x: f64, y: f64) -> (f64, f64) {
        self.transform.to_rad(x, y)
    }

    fn from_rad(&self, lon: f64, lat: f64) -> (f64, f64) {
       self.transform.from_rad(lon, lat)
    }
}

impl std::fmt::Debug for BoxedTransform{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedTransform").field("transform", &"[dyn Projection instance]").field("epsg_code", &self.epsg_code).finish()
    }
}