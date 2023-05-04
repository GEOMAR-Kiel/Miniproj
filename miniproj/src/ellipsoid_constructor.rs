//This file is licensed under EUPL v1.2

include!(concat!(env!("OUT_DIR"), "/ellipsoid_constructors.rs"));

pub fn get_ellipsoid(code: u32) -> Option<&'static Ellipsoid> {
    ELLIPSOIDS.get(&code).cloned()
}
