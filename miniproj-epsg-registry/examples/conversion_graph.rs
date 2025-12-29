use std::{collections::HashMap, fs::OpenOptions, io::Write};

use miniproj_epsg_registry::{Field, MemoryDb};

fn main() {
    let db = MemoryDb::new();
    let datums = db
        .get_table("epsg_datum")
        .unwrap()
        .get_rows(&[
            "datum_code",
            "datum_type",
            "ellipsoid_code",
            "prime_meridian_code",
        ])
        .unwrap()
        .filter_map(|fields| {
            if let [
                Some(Field::IntLike(datum_code)),
                Some(Field::StringLike("geodetic" | "dynamic geodetic")),
                Some(Field::IntLike(ellipsoid_code)),
                Some(Field::IntLike(8901)),
            ] = fields
            {
                Some((datum_code, ellipsoid_code))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let datum_ensembles = db
        .get_table("epsg_datumensemblemember")
        .unwrap()
        .get_rows(&["datum_ensemble_code", "datum_code"])
        .unwrap()
        .filter_map(|fields| {
            if let [
                Some(Field::IntLike(datum_ensemble_code)),
                Some(Field::IntLike(datum_code)),
            ] = fields
            {
                datums.get(&datum_code).map(|e| (datum_ensemble_code, *e))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let areas = db
        .get_table("epsg_extent")
        .unwrap()
        .get_rows(&[
            "extent_code",
            "bbox_west_bound_lon",
            "bbox_south_bound_lat",
            "bbox_east_bound_lon",
            "bbox_north_bound_lat",
        ])
        .unwrap()
        .filter_map(|fields| {
            if let [
                Some(Field::IntLike(extent_code)),
                Some(Field::Double(west)),
                Some(Field::Double(south)),
                Some(Field::Double(east)),
                Some(Field::Double(north)),
            ] = fields
            {
                Some((extent_code, ((west, south), (east, north))))
            } else {
                None
            }
        })
        .collect::<HashMap<i64, ((f64, f64), (f64, f64))>>();
    let geocentric_crs = db
        .get_table("epsg_coordinatereferencesystem")
        .unwrap()
        .get_rows(&["coord_ref_sys_code", "coord_ref_sys_kind", "datum_code"])
        .unwrap()
        .filter_map(|fields| {
            if let [
                Some(Field::IntLike(coord_ref_sys_code)),
                Some(Field::StringLike("geocentric")),
                Some(Field::IntLike(datum_code)),
            ] = fields
                && (datums.contains_key(&datum_code) || datum_ensembles.contains_key(&datum_code))
            {
                Some((coord_ref_sys_code, datum_code))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let geographic3d_crs = db
        .get_table("epsg_coordinatereferencesystem")
        .unwrap()
        .get_rows(&[
            "coord_ref_sys_code",
            "coord_ref_sys_kind",
            "base_crs_code",
            "projection_conv_code",
        ])
        .unwrap()
        .filter_map(|fields| {
            if let [
                Some(Field::IntLike(coord_ref_sys_code)),
                Some(Field::StringLike("geographic 3D")),
                Some(Field::IntLike(base_crs_code)),
                Some(Field::IntLike(15592)),
            ] = fields
                && geocentric_crs.contains_key(&base_crs_code)
            {
                Some((coord_ref_sys_code, base_crs_code))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let geographic2d_crs = db
        .get_table("epsg_coordinatereferencesystem")
        .unwrap()
        .get_rows(&[
            "coord_ref_sys_code",
            "coord_ref_sys_kind",
            "base_crs_code",
            "projection_conv_code",
        ])
        .unwrap()
        .filter_map(|fields| {
            if let [
                Some(Field::IntLike(coord_ref_sys_code)),
                Some(Field::StringLike("geographic 2D")),
                Some(Field::IntLike(base_crs_code)),
                Some(Field::IntLike(15593)),
            ] = fields
                && geographic3d_crs.contains_key(&base_crs_code)
            {
                Some((coord_ref_sys_code, base_crs_code))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let transformations = db
        .get_table("epsg_coordoperation")
        .unwrap()
        .get_rows(&[
            "coord_op_code",
            "coord_op_type",
            "source_crs_code",
            "target_crs_code",
            "coord_op_method_code",
        ])
        .unwrap()
        .filter_map(|fields| match fields {
            [
                Some(Field::IntLike(coord_op_code)),
                Some(Field::StringLike("transformation")),
                Some(Field::IntLike(source_crs_code)),
                Some(Field::IntLike(target_crs_code)),
                Some(Field::IntLike(1033 | 1032 | 1034 | 1061 | 1053 | 1056)), // geocentric to geocentric
            ] if geocentric_crs.contains_key(&source_crs_code)
                && geocentric_crs.contains_key(&target_crs_code) =>
            {
                Some((coord_op_code, (source_crs_code, target_crs_code)))
            }
            [
                Some(Field::IntLike(coord_op_code)),
                Some(Field::StringLike("transformation")),
                Some(Field::IntLike(source_crs_code)),
                Some(Field::IntLike(target_crs_code)),
                Some(Field::IntLike(1037 | 1038 | 1062 | 1039 | 1055 | 1058)), // geo3d to geo3d
            ] if geographic3d_crs.contains_key(&source_crs_code)
                && geographic3d_crs.contains_key(&target_crs_code) =>
            {
                Some((coord_op_code, (source_crs_code, target_crs_code)))
            }
            [
                Some(Field::IntLike(coord_op_code)),
                Some(Field::StringLike("transformation")),
                Some(Field::IntLike(source_crs_code)),
                Some(Field::IntLike(target_crs_code)),
                Some(Field::IntLike(1063 | 9636 | 9607 | 9606 | 1054 | 1057)), // geo2d to geo2d
            ] if geographic2d_crs.contains_key(&source_crs_code)
                && geographic2d_crs.contains_key(&target_crs_code) =>
            {
                Some((coord_op_code, (source_crs_code, target_crs_code)))
            }
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("coord_ops.dot")
        .unwrap();
    f.write_all(b"graph G { overlap_scaling=-8 beautify=true\n")
        .unwrap();
    for (id, datum) in geocentric_crs {
        f.write_all(
            format!("crs{id} [shape=Mrecord label=\"{id}|Geocentric|Datum: {datum}\"]\n")
                .as_bytes(),
        )
        .unwrap();
    }
    for (id, base) in &geographic3d_crs {
        f.write_all(format!("crs{id} [shape=Mrecord label=\"{id}|Geographic3D\"]\n").as_bytes())
            .unwrap();
        f.write_all(format!("crs{id} -- crs{base}\n").as_bytes())
            .unwrap();
    }
    for (id, base) in &geographic2d_crs {
        f.write_all(format!("crs{id} [shape=Mrecord label=\"{id}|Geographic2D\"]\n").as_bytes())
            .unwrap();
        f.write_all(format!("crs{id} -- crs{base}\n").as_bytes())
            .unwrap();
    }
    for (id, (src, tgt)) in transformations {
        let (src, tgt) = if let (Some(src), Some(tgt)) =
            (geographic2d_crs.get(&src), geographic2d_crs.get(&tgt))
        {
            (*src, *tgt)
        } else {
            (src, tgt)
        };
        let (src, tgt) = if let (Some(src), Some(tgt)) =
            (geographic3d_crs.get(&src), geographic3d_crs.get(&tgt))
        {
            (*src, *tgt)
        } else {
            (src, tgt)
        };
        f.write_all(format!("crs{src} -- crs{tgt} [label=\"{id}\"]\n").as_bytes())
            .unwrap();
    }
    f.write_all(b"}").unwrap();
}
