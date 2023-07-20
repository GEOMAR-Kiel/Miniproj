//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

mod db;
mod helpers;
mod sql;

pub use crate::db::*;
pub use crate::sql::*;
use miniproj_ops::ellipsoid::Ellipsoid;

type ImplementedProjection = (
    u32,
    &'static (dyn (Fn(&[(u32, f64)], Ellipsoid) -> String) + Send + Sync),
);

/// Implemented projections.
///
/// Pairs operation codes with functions that map a slice of (parameter code, value)-tuples and an ellipsoid
/// to a `String` containing source code for constructing the `Projection` with the given parameters.
pub static IMPL_CONV: &[ImplementedProjection] = &[
    (9807, &miniproj_ops::transverse_mercator::direct_projection),
    (
        9820,
        &miniproj_ops::lambert_azimuthal_equal_area::direct_projection,
    ),
    (9810, &miniproj_ops::stereographic::direct_projection_a),
    (
        9802,
        &miniproj_ops::lambert_conic_conformal::direct_projection_2sp,
    ),
    (
        1024,
        &miniproj_ops::popvis_pseudo_mercator::direct_projection,
    ),
    (
        9801,
        &miniproj_ops::lambert_conic_conformal::direct_projection_1sp_a,
    ),
    (
        9809,
        &miniproj_ops::stereographic::direct_projection_oblique,
    ),
    (9822, &miniproj_ops::albers_equal_area::direct_projection),
];

#[cfg(test)]
mod tests {
    use crate::sql::MemoryDb;

    #[test]
    fn create_mem_db() {
        let memdb = MemoryDb::new();
        eprintln!("{memdb:#?}")
    }
}
