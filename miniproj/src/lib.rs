//This file is licensed under EUPL v1.2
#![feature(const_float_bits_conv)]


#![doc = include_str!("../README.md")]


mod projection_constructor;
mod ellipsoid_constructor;

#[doc(inline)]
pub use miniproj_ops::Projection;
#[doc(inline)]
pub use projection_constructor::get_projection;