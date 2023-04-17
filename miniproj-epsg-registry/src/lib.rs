//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

mod db;
mod helpers;
use std::path::Path;

pub use crate::db::*;
pub use rusqlite::Connection as DbConnection;
use miniproj_ops::ellipsoid::Ellipsoid;

type ImplementedConversion = (u32, &'static (dyn (Fn(&[(u32, f64)], Ellipsoid) -> String) + Send + Sync));

/// Implemented conversions.
/// 
/// Pairs operation codes with a functions that map a slice of (parameter code, value)-tuples and an ellipsoid
/// to a `String` containing source code for constructing the `CoordTransform` with the given parameters.
pub static IMPL_CONV: &[ImplementedConversion] = &[
    (9807, &miniproj_ops::transverse_mercator::direct_conversion),
    (9820, &miniproj_ops::lambert_azimuthal_equal_area::direct_conversion),
    (9810, &miniproj_ops::stereographic::direct_conversion_a),
    (9802, &miniproj_ops::lambert_conic_conformal::direct_conversion_2sp),
    (1024, &miniproj_ops::popvis_pseudo_mercator::direct_conversion),
];

/// This function copies the parameter database to the given location, to reliably make it available to build scripts.
pub fn write_db<P: AsRef<Path>>(path: P) -> std::io::Result<()>{
    std::fs::write(path, include_bytes!("../data/parameters.sqlite"))
}


#[cfg(test)]
mod tests {

}
