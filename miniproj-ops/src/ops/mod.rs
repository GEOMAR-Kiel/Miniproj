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

/// Try to construct a projection for a specific method code with a getter that provides the parameter values.
/// 
/// Note that despite taking a reference to an ellipsoid the resulting projection will not update when the ellipsoid is altered.
/// Reconstruct the projection if you need it for a different ellipsoid.
/// Similarly, the getter is called once per required parameter on construction, and in no guaranteed order.
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
