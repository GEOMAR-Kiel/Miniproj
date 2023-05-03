use miniproj_ops::Ellipsoid;

include!(concat!(env!("OUT_DIR"), "/ellipsoid_constructors.rs"));

/// Returns the Projection corresponding to the EPSG code passed as the argument.
/// If the code refers to a projection that is not implemented, the method returns None
pub fn get_ellipsoid(code: u32) -> Option<&'static Ellipsoid> {
    ELLIPSOIDS.get(&code)
}
