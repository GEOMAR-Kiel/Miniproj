//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

mod db;
mod helpers;
pub use crate::db::*;
pub use rusqlite::Connection as DbConnection;
use epsg_coordoperations::ellipsoid::Ellipsoid;

type ImplementedConversion = (u32, &'static (dyn (Fn(&[(u32, f64)], Ellipsoid) -> String) + Send + Sync));

pub static IMPL_CONV: &[(u32, &(dyn (Fn(&[(u32, f64)], Ellipsoid) -> String) + Send + Sync))] = &[
    (9807, &epsg_coordoperations::transverse_mercator::direct_conversion),
    (9820, &epsg_coordoperations::lambert_azimuthal_equal_area::direct_conversion)
];


#[cfg(test)]
mod tests {

}
