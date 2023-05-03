use miniproj_ops::Ellipsoid;

include!(concat!(env!("OUT_DIR"), "/ellipsoid_constructors.rs"));

/// Returns the Ellipsoid corresponding to the EPSG code passed as the argument.
/// If the code does not refer to an Ellipsoid, the method returns `None`.
pub fn get_ellipsoid(code: u32) -> Option<&'static Ellipsoid> {
    ELLIPSOIDS.get(&code)
}
