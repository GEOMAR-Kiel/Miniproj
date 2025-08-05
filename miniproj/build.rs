//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use miniproj_epsg_registry::*;
use std::env;
use std::path::PathBuf;

fn main() {
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut projection_out = output_dir.clone();
    let memdb = MemoryDb::new();
    dump_crs_relations(&memdb);
    let ellipsoids = get_ellipsoids(&memdb).unwrap();
    projection_out.push("projection_constructors.rs");
    std::fs::write(
        projection_out,
        gen_parameter_constructors(&memdb, IMPL_CONV, &ellipsoids).unwrap(),
    )
    .unwrap();
    let mut ellipsoid_out = output_dir;
    ellipsoid_out.push("ellipsoid_constructors.rs");
    std::fs::write(ellipsoid_out, gen_ellipsoid_constructors(&memdb).unwrap()).unwrap();
}
