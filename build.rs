//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::env;
use std::path::PathBuf;
use epsg_geodetic_parameter_gen::*;

fn main() {
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut data_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("epsg-geodetic-parameter-gen/data/parameters.sqlite");
    let db = DbConnection::open(data_path).unwrap();
    let mut ellipsoid_out = output_dir.clone();
    ellipsoid_out.push("ellipsoid_constructors.rs");
    std::fs::write(ellipsoid_out , gen_ellipsoids_source(&db).unwrap()).unwrap();
    let mut conversion_out = output_dir.clone();
    conversion_out.push("projection_constructors.rs");
    std::fs::write(conversion_out, gen_parameter_constructors(&db, &IMPL_CONV).unwrap()).unwrap();
}