//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::env;
use std::path::PathBuf;
use miniproj_epsg_registry::*;

fn main() {
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut data_path = output_dir.clone();
    data_path.push("parameters.sqlite");
    write_db(&data_path).expect("Could not write db object");
    let db = DbConnection::open(data_path).unwrap();
    let mut projection_out = output_dir;
    let ellipsoids = get_ellipsoids(&db).unwrap();
    projection_out.push("projection_constructors.rs");
    std::fs::write(projection_out, gen_parameter_constructors(&db, IMPL_CONV, &ellipsoids).unwrap()).unwrap();
}