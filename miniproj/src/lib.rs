//This file is licensed under EUPL v1.2
#![doc = include_str!("../README.md")]

mod ellipsoid_constructor;
mod projection_constructor;

#[doc(inline)]
pub use ellipsoid_constructor::get_ellipsoid;
#[doc(inline)]
pub use miniproj_ops::custom_projection;

#[doc(inline)]
pub use miniproj_ops::{Ellipsoid, Projection};
#[doc(inline)]
pub use projection_constructor::{get_ellipsoid_code, get_projection, create_projection};
