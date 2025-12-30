use crate::{
    Ellipsoid, Projection,
    albers_equal_area::AlbersEqualAreaParams,
    lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaParams,
    lambert_conic_conformal::{LambertConic1SPAParams, LambertConic2SPParams},
    popvis_pseudo_mercator::PopVisPseudoMercatorParams,
    stereographic::{ObliqueStereographicParams, PolarStereographicAParams},
    transverse_mercator::TransverseMercatorParams,
};

use self::{
    albers_equal_area::AlbersEqualAreaProjection,
    lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaProjection,
    lambert_conic_conformal::{LambertConic1SPAProjection, LambertConic2SPProjection},
    popvis_pseudo_mercator::PopVisPseudoMercatorProjection,
    stereographic::{ObliqueStereographicProjection, PolarStereographicAProjection},
    transverse_mercator::TransverseMercatorProjection,
};

pub mod ellipsoid;

pub mod albers_equal_area;
pub mod helmert;
pub mod identity_projection;
pub mod lambert_azimuthal_equal_area;
pub mod lambert_conic_conformal;
pub mod molodensky_badekas;
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
    let params = param_builder(pmethod_code, getter)?;
    Some(params.to_projection(ellipsoid))
}

pub enum ProjectionParams {
    TransverseMercator(TransverseMercatorParams),
    PolarStereographicA(PolarStereographicAParams),
    LambertConic2SP(LambertConic2SPParams),
    PopVisPseudoMercator(PopVisPseudoMercatorParams),
    LambertConic1SPA(LambertConic1SPAParams),
    ObliqueStereographic(ObliqueStereographicParams),
    AlbersEqualArea(AlbersEqualAreaParams),
    LambertAzimuthalEqualArea(LambertAzimuthalEqualAreaParams),
}
impl ProjectionParams {
    fn to_projection(&self, ell: &Ellipsoid) -> Box<dyn Projection> {
        match self {
            ProjectionParams::TransverseMercator(params) => {
                Box::new(TransverseMercatorProjection::new(ell, params))
            }
            ProjectionParams::PolarStereographicA(params) => {
                Box::new(PolarStereographicAProjection::new(ell, params))
            }
            ProjectionParams::LambertConic2SP(params) => {
                Box::new(LambertConic2SPProjection::new(ell, params))
            }
            ProjectionParams::PopVisPseudoMercator(params) => {
                Box::new(PopVisPseudoMercatorProjection::new(ell, params))
            }
            ProjectionParams::LambertConic1SPA(params) => {
                Box::new(LambertConic1SPAProjection::new(ell, params))
            }
            ProjectionParams::ObliqueStereographic(params) => {
                Box::new(ObliqueStereographicProjection::new(ell, params))
            }
            ProjectionParams::AlbersEqualArea(params) => {
                Box::new(AlbersEqualAreaProjection::new(ell, params))
            }
            ProjectionParams::LambertAzimuthalEqualArea(params) => {
                Box::new(LambertAzimuthalEqualAreaProjection::new(ell, params))
            }
        }
    }
}

#[cfg(feature = "codegen")]
impl ProjectionParams {
    pub fn to_constructor(&self) -> String {
        match self {
            ProjectionParams::TransverseMercator(params) => format!(
                "ProjectionParams::TransverseMercator({})",
                params.to_constructor()
            ),
            ProjectionParams::PolarStereographicA(params) => format!(
                "ProjectionParams::PolarStereographicA({})",
                params.to_constructor()
            ),

            ProjectionParams::LambertConic2SP(params) => format!(
                "ProjectionParams::LambertConic2SP({})",
                params.to_constructor()
            ),

            ProjectionParams::PopVisPseudoMercator(params) => format!(
                "ProjectionParams::PopVisPseudoMercator({})",
                params.to_constructor()
            ),

            ProjectionParams::LambertConic1SPA(params) => format!(
                "ProjectionParams::LambertConic1SPA({})",
                params.to_constructor()
            ),

            ProjectionParams::ObliqueStereographic(params) => format!(
                "ProjectionParams::ObliqueStereographic({})",
                params.to_constructor()
            ),

            ProjectionParams::AlbersEqualArea(params) => format!(
                "ProjectionParams::AlbersEqualArea({})",
                params.to_constructor()
            ),

            ProjectionParams::LambertAzimuthalEqualArea(params) => format!(
                "ProjectionParams::LambertAzimuthalEqualArea({})",
                params.to_constructor()
            ),
        }
    }
}

pub fn param_builder<G>(pmethod_code: u32, getter: G) -> Option<ProjectionParams>
where
    G: FnMut(u32) -> Option<f64>,
{
    use crate::types::DbContstruct;

    match pmethod_code {
        9807 => Some(ProjectionParams::TransverseMercator(
            TransverseMercatorParams::from_db(getter)?,
        )),
        9810 => Some(ProjectionParams::PolarStereographicA(
            PolarStereographicAParams::from_db(getter)?,
        )),
        9802 => Some(ProjectionParams::LambertConic2SP(
            LambertConic2SPParams::from_db(getter)?,
        )),
        1024 => Some(ProjectionParams::PopVisPseudoMercator(
            PopVisPseudoMercatorParams::from_db(getter)?,
        )),
        9801 => Some(ProjectionParams::LambertConic1SPA(
            LambertConic1SPAParams::from_db(getter)?,
        )),
        9809 => Some(ProjectionParams::ObliqueStereographic(
            ObliqueStereographicParams::from_db(getter)?,
        )),
        9822 => Some(ProjectionParams::AlbersEqualArea(
            AlbersEqualAreaParams::from_db(getter)?,
        )),
        9820 => Some(ProjectionParams::LambertAzimuthalEqualArea(
            LambertAzimuthalEqualAreaParams::from_db(getter)?,
        )),
        _ => None,
    }
}
