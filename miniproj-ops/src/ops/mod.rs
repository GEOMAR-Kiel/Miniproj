use crate::{traits::GetterContstruct, Ellipsoid, Projection};

use self::{
    albers_equal_area::AlbersEqualAreaProjection,
    lambert_conic_conformal::{LambertConic1SPAProjection, LambertConic2SPProjection},
    popvis_pseudo_mercator::PopVisPseudoMercatorProjection,
    stereographic::{ObliqueStereographicProjection, PolarStereographicAProjection},
    transverse_mercator::TransverseMercatorProjection,
};

pub mod ellipsoid;

pub mod albers_equal_area;
pub mod identity_projection;
pub mod lambert_azimuthal_equal_area;
pub mod lambert_conic_conformal;
pub mod popvis_pseudo_mercator;
pub mod stereographic;
pub mod transverse_mercator;

type ImplementedProjection = (
    u32,
    &'static (dyn (Fn(&[(u32, f64)], Ellipsoid) -> String) + Send + Sync),
);

/// Implemented projections.
///
/// Pairs operation codes with functions that map a slice of (parameter code, value)-tuples and an ellipsoid
/// to a `String` containing source code for constructing the `Projection` with the given parameters.
pub static IMPL_CONV: &[ImplementedProjection] = &[
    (9807, &transverse_mercator::direct_projection),
    (9820, &lambert_azimuthal_equal_area::direct_projection),
    (9810, &stereographic::direct_projection_a),
    (9802, &lambert_conic_conformal::direct_projection_2sp),
    (1024, &popvis_pseudo_mercator::direct_projection),
    (9801, &lambert_conic_conformal::direct_projection_1sp_a),
    (9809, &stereographic::direct_projection_oblique),
    (9822, &albers_equal_area::direct_projection),
];

pub fn custom_projection<G>(
    pmethod_code: u32,
    getter: G,
    ellipsoid: &Ellipsoid,
) -> Option<Box<dyn Projection>>
where
    G: FnMut(u32) -> Option<f64>,
{
    match pmethod_code {
        9807 => Some(Box::new(TransverseMercatorProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        9810 => Some(Box::new(PolarStereographicAProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        9802 => Some(Box::new(LambertConic2SPProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        1024 => Some(Box::new(PopVisPseudoMercatorProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        9801 => Some(Box::new(LambertConic1SPAProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        9809 => Some(Box::new(ObliqueStereographicProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        9822 => Some(Box::new(AlbersEqualAreaProjection::with_db_getter(
            getter, ellipsoid,
        )?)),
        _ => None,
    }
}
