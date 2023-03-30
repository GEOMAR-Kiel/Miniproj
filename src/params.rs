use epsg_coordoperations::{lambert_azimuthal_equal_area::LambertAzimuthalEqualAreaParams, transverse_mercator::{TransverseMercatorParams, TransverseMercatorConversion}, stereographic::{PolarStereographicAParams, PolarStereographicAConversion}, ellipsoid::Ellipsoid, CoordTransform};

#[derive(Copy, Clone, Debug)]
pub enum Params{
    LambertAzimuthalEqualAreaParams(LambertAzimuthalEqualAreaParams),
    TransverseMercatorParams(TransverseMercatorParams),
    PolarStereographicAParams(PolarStereographicAParams)
}

impl Params{
    pub fn apply_named_conversion(&'static self, ellipsoid: &'static Ellipsoid, conversion: &'static str) -> Box<dyn CoordTransform>{
        match conversion {
            "TransverseMercatorConversion" => Box::new(TransverseMercatorConversion::new(ellipsoid, self.into())),
            "PolarStereographicAConversion" => Box::new(PolarStereographicAConversion::new(ellipsoid, self.into())),
            _ => panic!("Unknown conversion: {conversion}")
        }
    }
}

impl From<&'static Params> for &'static LambertAzimuthalEqualAreaParams{
    fn from(value: &'static Params) -> Self {
        match value{
            Params::LambertAzimuthalEqualAreaParams(laeap) => laeap,
            _ => panic!("Cannot convert params")
        }
    }
}

impl From<&'static Params> for &'static TransverseMercatorParams{
    fn from(value: &'static Params) -> Self {
        match value{
            Params::TransverseMercatorParams(tmp) => tmp,
            _ => panic!("Cannot convert params")
        }
    }
}

impl From<&'static Params> for &'static PolarStereographicAParams{
    fn from(value: &'static Params) -> Self {
        match value{
            Params::PolarStereographicAParams(psap) => psap,
            _ => panic!("Cannot convert params")
        }
    }
}