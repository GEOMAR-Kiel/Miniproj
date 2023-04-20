//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer
#![feature(const_float_bits_conv)]


#![doc = include_str!("../README.md")]


mod projection_constructor;

#[doc(inline)]
pub use miniproj_ops::Projection;
#[doc(inline)]
pub use projection_constructor::get_projection;