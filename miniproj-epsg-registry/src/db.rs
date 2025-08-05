//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::OpenOptions,
    io::Write,
    num::TryFromIntError, u32,
};

use crate::{
    helpers::*,
    sql::{Field, MemoryDb},
    ImplementedProjection,
};
use miniproj_ops::ellipsoid::Ellipsoid;
use miniproj_ops::PseudoSerialize;

/// Generates rust source code mapping EPSG codes to `Ellipsoid`s.
pub fn gen_ellipsoid_constructors(db: &MemoryDb) -> Result<String, Box<dyn Error>> {
    let ell_rows = db
        .get_table("epsg_ellipsoid")
        .ok_or("No Ellipsoid Table")?
        .get_rows(&[
            "ellipsoid_code",
            "semi_major_axis",
            "semi_minor_axis",
            "inv_flattening",
            "uom_code",
        ])?
        .collect::<Vec<_>>();
    let uom_rows = db
        .get_table("epsg_unitofmeasure")
        .ok_or("No UOM Table")?
        .get_rows(&["uom_code", "factor_b", "factor_c"])?
        .collect::<Vec<_>>();

    let mut constant_defs: String = String::from(
        "#[allow(clippy::approx_constant)]\nstatic ELLIPSOIDS: phf::Map<u32, Ellipsoid> =",
    );
    let mut phf_map = phf_codegen::Map::new();
    for a in &ell_rows {
        let [Some(Field::IntLike(code)), _, _, _, Some(Field::IntLike(uom_code))] = a else {
            unreachable!("No UOM Code given. (row: {:?})", a)
        };
        let Some([_, Some(Field::Double(fac_b)), Some(Field::Double(fac_c))]) =
            uom_rows.iter().find(|[f, _, _]| {
                if let Some(Field::IntLike(code)) = f {
                    code == uom_code
                } else {
                    false
                }
            })
        else {
            unreachable!("No UOM found for Code in DB.")
        };
        let ellipsoid = match a {
            [_, Some(Field::Double(a)), Some(Field::Double(b)), None, _] => {
                Ellipsoid::from_a_b(a * fac_b / fac_c, b * fac_b / fac_c)
            }
            [_, Some(Field::Double(a)), None, Some(Field::Double(f_inv)), _] => {
                Ellipsoid::from_a_f_inv(a * fac_b / fac_c, *f_inv)
            }
            _ => unreachable!("Malformed DB: Ellipsoids need either b or f_inv. (row: {a:?}"),
        };
        phf_map.entry(u32::try_from(*code)?, &ellipsoid.to_constructed());
    }
    constant_defs.push_str(&phf_map.build().to_string());
    constant_defs.push(';');
    Ok(constant_defs)
}

/// Constructs a `HashMap` mapping EPSG codes to `Ellipsoid`s.
pub fn get_ellipsoids(db: &MemoryDb) -> Result<HashMap<u32, Ellipsoid>, Box<dyn Error>> {
    let ell_rows = db
        .get_table("epsg_ellipsoid")
        .ok_or("No Ellipsoid Table")?
        .get_rows(&[
            "ellipsoid_code",
            "semi_major_axis",
            "semi_minor_axis",
            "inv_flattening",
            "uom_code",
        ])?
        .collect::<Vec<_>>();
    let uom_rows = db
        .get_table("epsg_unitofmeasure")
        .ok_or("No UOM Table")?
        .get_rows(&["uom_code", "factor_b", "factor_c"])?
        .collect::<Vec<_>>();

    let mut ellipsoids = HashMap::new();
    for a in &ell_rows {
        let [Some(Field::IntLike(code)), _, _, _, Some(Field::IntLike(uom_code))] = a else {
            return Err(format!("No UOM Code given. (row: {:?})", a).into());
        };
        let Some([_, Some(Field::Double(fac_b)), Some(Field::Double(fac_c))]) =
            uom_rows.iter().find(|[f, _, _]| {
                if let Some(Field::IntLike(code)) = f {
                    code == uom_code
                } else {
                    false
                }
            })
        else {
            unreachable!("No UOM found for Code in DB.")
        };
        let ellipsoid = match a {
            [_, Some(Field::Double(a)), Some(Field::Double(b)), None, _] => {
                Ellipsoid::from_a_b(a * fac_b / fac_c, b * fac_b / fac_c)
            }
            [_, Some(Field::Double(a)), None, Some(Field::Double(f_inv)), _] => {
                Ellipsoid::from_a_f_inv(a * fac_b / fac_c, *f_inv)
            }
            _ => {
                return Err(
                    format!("Malformed DB: Ellipsoids need either b or f_inv. (row: {a:?}").into(),
                )
            }
        };
        ellipsoids.insert((*code).try_into()?, ellipsoid);
    }
    Ok(ellipsoids)
}

/// Generates rust source code mapping EPSG codes to prime meridian angles in radians relative to the Greenwich meridian.
pub fn gen_prime_meridians_source(_c: &MemoryDb) -> Result<String, Box<dyn Error>> {
    todo!()
}

/// Constructs a `HashMap` mapping EPSG codes to prime meridian angles in radians relative to the Greenwich meridian.
pub fn get_prime_meridians(_c: &MemoryDb) -> Result<HashMap<u32, f64>, Box<dyn Error>> {
    todo!()
}

#[derive(Debug)]
enum CrsEntry {
    Geographic2D { datum: u32 },
    Projected { conversion: u32, base: u32 },
}

#[derive(PartialEq, Eq, Hash)]
struct CoordOp {
    code: u32,
    from: u32,
    to: u32,
    method: u32,
}

pub fn dump_crs_relations(db: &MemoryDb) {
    const PERMITTED_METHODS: &'static [i64] = &[
        // Conversions
        9602, // Geographic3D->Geocentric
        9659, // Geographic3D->Geographic2D
        // Transformations
        1061, // Molodensky-Badekas PV
        1062, // Molodensky-Badekas PV Geographic3D Concatenated
        1063, // Molodensky-Badekas PV Geographic2D Concatenated
        1034, // Molodensky-Badekas CF
        1039, // Molodensky-Badekas CF Geographic3D Concatenated
        9636, // Molodensky-Badekas CF Geographic2D Concatenated
        1033, // Position Vector TF
        1037, // Position Vector TF Geographic3D Concatenated
        9606, // Position Vector TF Geographic2D Concatenated
        1032, // CF Rotation
        1038, // CF Rotation Geographic3D Concatenated
        9607, // CF Rotation Geographic2D Concatenated
        1053, // Time-dependent PV
        1055, // Time-dependent PV Geographic3D Concatenated
        1054, // Time-dependent PV Geographic2D Concatenated
        1056, // Time-dependent CF rotation
        1058, // Time-dependent CF rotation Geographic3D Concatenated
        1057, // Time-dependent CF rotation Geographic2D Concatenated
        1031, // Translation
        1035, // Translation Geographic3D Concatenated
        9603, // Translation Geographic2D Concatenated
        1064, // Point Motion
        1067, // Point Motion Ellipsoidal
        1065, // Time-specific PV
        1066, // Time-specific CF
        // Projections
        1024, // PopVisPseudoMercator
        9801, // LambertConic1SPA
        9802, // LambertConic2SPP
        9807, // TransverseMercator
        9809, // ObliqueStereographic,
        9810, // PolarStereographicA
        9820, // LAEA
        9822, // AlbersEqualArea
    ];

    fn recurse_graph(db: &MemoryDb, nodes: &mut HashSet<u32>, edges: &mut HashSet<CoordOp>, start: i64) {
        let op_table = db.get_table("epsg_coordoperation").unwrap();
        for row in op_table.get_rows_where_i64("source_crs_code", start, &[
            "coord_op_code",
            "source_crs_code",
            "target_crs_code",
            "coord_op_method_code",
        ]).into_iter().chain(op_table.get_rows_where_i64("target_crs_code", start, &[
            "coord_op_code",
            "source_crs_code",
            "target_crs_code",
            "coord_op_method_code",
        ]).into_iter()) {
            let [Some(Field::IntLike(op_code)), Some(Field::IntLike(from)), Some(Field::IntLike(to)), Some(Field::IntLike(method))] = row else {continue};
            edges.insert(CoordOp { code: op_code as u32, from: from as u32, to: to as u32, method: method as u32 });
            if !PERMITTED_METHODS.contains(&method) {
                println!("cargo:warning=Method {method} is not permitted (expanding from {start})");
                continue;
            }
            edges.insert(CoordOp { code: op_code as u32, from: from as u32, to: to as u32, method: method as u32 });
            if nodes.insert(from as u32) {
                recurse_graph(db, nodes, edges, from);
            }
            if nodes.insert(to as u32) {
                recurse_graph(db, nodes, edges, to);
            } 
        }
        let ref_sys_table = db.get_table("epsg_coordinatereferencesystem").unwrap();
        for row in ref_sys_table.get_rows_where_i64("base_crs_code", start, &["coord_ref_sys_code", "coord_ref_sys_kind", "projection_conv_code"]) {
            let [Some(Field::IntLike(from)), Some(Field::StringLike(kind)), conv] = row else {continue};
            match (kind, conv) {
                ("projected", Some(Field::IntLike(code))) => {
                    edges.insert(CoordOp { code: code as u32, from: from as u32, to: start as u32, method: code as u32});
                    continue;
                }
                ("geographic 2D", Some(Field::IntLike(code))) => {
                    edges.insert(CoordOp { code: u32::MAX, from: from as u32, to: start as u32, method: code as u32 });
                }
                ("geographic 3D", Some(Field::IntLike(code))) => {
                    edges.insert(CoordOp { code: u32::MAX, from: from as u32, to: start as u32, method: code as u32 });
                },
                _ => continue
            }
            if nodes.insert(from as u32) {
                recurse_graph(db, nodes, edges, from);
            } 
        }
    }


    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();

    recurse_graph(db, &mut nodes, &mut edges, 4936);

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("coord_ops.dot")
        .unwrap();
    f.write_all(b"digraph G { overlap_scaling=-8 beautify=true\n").unwrap();
    for n in nodes {
        f.write_all(format!("crs{n} [label=\"{n}\"]\n").as_bytes())
            .unwrap();
    }
    for CoordOp {
        code,
        from,
        to,
        method,
    } in edges
    {
        f.write_all(
            format!("crs{from} -> crs{to} [label=\"{code}\nusing {method}\"]\n").as_bytes(),
        )
        .unwrap();
    }
    f.write_all(b"}").unwrap();
}

/// Generates rust source code for projected and geographic coordinate systems for all implemented projections.
pub fn gen_parameter_constructors(
    db: &MemoryDb,
    supporteds: &[ImplementedProjection],
    ellipsoids: &HashMap<u32, Ellipsoid>,
) -> Result<String, Box<dyn Error>> {
    let units = db.get_table("epsg_unitofmeasure")
        .ok_or("No UOM table")?
        .get_rows(&["uom_code", "factor_b", "factor_c"])?
        .filter_map(|row| {
            match row {
                [Some(Field::IntLike(uom_code)), Some(Field::Double(factor_b)), Some(Field::Double(factor_c))] =>
                    Some((u32::try_from(uom_code).ok()?, (factor_b, factor_c))),
                _ => None
            }
        }).collect::<HashMap<u32, _>>();

    let crs_table = db.get_table("epsg_coordinatereferencesystem")
        .ok_or("No CRS table")?
        .get_rows(&["coord_ref_sys_code", "base_crs_code", "projection_conv_code", "datum_code", "coord_ref_sys_kind"])?
        .filter_map(|row| {
            match row {
                [Some(Field::IntLike(code)), _, _, Some(Field::IntLike(datum_code)), Some(Field::StringLike("geographic 2D"))] => {
                    Some((u32::try_from(code).ok()?, CrsEntry::Geographic2D { datum: u32::try_from(datum_code).ok()? }))
                },
                [Some(Field::IntLike(code)), Some(Field::IntLike(base_crs_code)), Some(Field::IntLike(conv_code)), _, Some(Field::StringLike("projected"))] => {
                    Some((u32::try_from(code).ok()?, CrsEntry::Projected { conversion: u32::try_from(conv_code).ok()?, base: u32::try_from(base_crs_code).ok()? }))
                },
                [Some(Field::IntLike(code)), _, _, _, Some(Field::StringLike(s))] => {
                    //println!("cargo:warning=CRS {code} has type \"{s}\"");
                    None
                },
                _ => None
            }
        })
        .collect::<HashMap<u32, _>>();
    assert!(!crs_table.is_empty());
    let names_table = db
        .get_table("epsg_coordinatereferencesystem")
        .ok_or("No CRS table")?
        .get_rows(&["coord_ref_sys_code", "coord_ref_sys_name"])?
        .filter_map(|row| match row {
            [Some(Field::IntLike(code)), Some(Field::StringLike(name))] => {
                Some((u32::try_from(code).ok()?, name))
            }
            _ => None,
        })
        .collect::<HashMap<u32, _>>();
    let extents_table = db.get_table("epsg_extent")
        .ok_or("No Extent Table")?
        .get_rows(&["extent_code", "extent_name", "bbox_south_bound_lat", "bbox_west_bound_lon", "bbox_north_bound_lat", "bbox_east_bound_lon"])?
        .filter_map(|row| {
            match row {
                [Some(Field::IntLike(code)), Some(Field::StringLike(name)), Some(Field::Double(lat_s)), Some(Field::Double(lon_w)), Some(Field::Double(lat_n)), Some(Field::Double(lon_e))] => {
                    Some((u32::try_from(code).ok()?, (name, [lon_e, lat_n, lon_w, lat_s]))) //TODO make a real type
                },
                _ => None
            }
        })
        .collect::<HashMap<u32, _>>();
    let mut usages_table: HashMap<u32, Vec<_>> = HashMap::new();
    db.get_table("epsg_usage")
        .ok_or("No Usage Table")?
        .get_rows(&["object_code", "extent_code"])?
        .for_each(|row| match row {
            [Some(Field::IntLike(object_code)), Some(Field::IntLike(extent_code))] => {
                let Ok(object_code) = u32::try_from(object_code) else {
                    return;
                };
                let Ok(extent_code) = u32::try_from(extent_code) else {
                    return;
                };
                if let Some((name, area)) = extents_table.get(&extent_code) {
                    usages_table
                        .entry(object_code)
                        .or_default()
                        .push((name, area))
                }
            }
            _ => {}
        });

    let op_table = db
        .get_table("epsg_coordoperation")
        .ok_or("No Op table")?
        .get_rows(&["coord_op_code", "coord_op_method_code"])?
        .filter_map(|row| {
            let [Some(Field::IntLike(coord_op_code)), Some(Field::IntLike(coord_op_method_code))] =
                row
            else {
                return None;
            };
            match (
                u32::try_from(coord_op_code),
                u32::try_from(coord_op_method_code),
            ) {
                (Ok(coord_op_code), Ok(coord_op_method_code)) => {
                    Some(Ok((coord_op_code, coord_op_method_code)))
                }
                (Err(e), _) | (_, Err(e)) => Some(Err(e)),
            }
        })
        .collect::<Result<HashMap<u32, u32>, TryFromIntError>>()?;

    let mut paramvalues: HashMap<u32, Vec<_>> = HashMap::new();
    db.get_table("epsg_coordoperationparamvalue")
            .ok_or("No Param Value table")?
            .get_rows(&["coord_op_code", "parameter_code", "parameter_value", "uom_code"])?
            .try_for_each::<_, Result<_, Box<dyn Error>>>(|row| {

                match row {
                    [Some(Field::IntLike(coord_op_code)), Some(Field::IntLike(parameter_code)), Some(Field::Double(v)), Some(Field::IntLike(9110))] => {
                        paramvalues.entry(u32::try_from(coord_op_code)?).or_default().push((u32::try_from(parameter_code)?, epsg_9110_to_rad(v)));
                    },
                    [Some(Field::IntLike(coord_op_code)), Some(Field::IntLike(parameter_code)), Some(Field::Double(v)), Some(Field::IntLike(uom_code))] => {
                        if let Some((factor_b, factor_c)) = units.get(&u32::try_from(uom_code)?) {
                            paramvalues.entry(u32::try_from(coord_op_code)?).or_default().push((u32::try_from(parameter_code)?, v * factor_b / factor_c));
                        }
                    },
                    //e => return Err(format!("Missing param values in {e:?}").into()),
                    _ => {}
                };
                Ok(())
            })?;

    assert!(!op_table.is_empty());
    let datum_table = db
        .get_table("epsg_datum")
        .ok_or("No Datum table")?
        .get_rows(&["datum_code", "ellipsoid_code", "prime_meridian_code"])?
        .filter_map(|row| {
            let [Some(Field::IntLike(code)), Some(Field::IntLike(ellipsoid_code)), Some(Field::IntLike(prime_meridian_code))] = row else {return None};
            match(u32::try_from(code), u32::try_from(ellipsoid_code), u32::try_from(prime_meridian_code)) {
                (Ok(code), Ok(ellipsoid_code), Ok(8901)) => { // since correction for other meridians is currently missing.
                    if ellipsoids.contains_key(&ellipsoid_code) {
                        Some(Ok((code, (ellipsoid_code, 8901))))
                    } else {
                        None
                    }
                },
                (Ok(_), Ok(_), Ok(_)) => None,
                (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => Some(Err(e))
            }
        }).collect::<Result<HashMap<u32, _>, TryFromIntError>>()?;

    let mut datum_ensemble_member_table: HashMap<u32, Vec<u32>> = HashMap::new();
    for r in db
        .get_table("epsg_datumensemblemember")
        .ok_or("No Datum Ensemble Member table")?
        .get_rows(&["datum_ensemble_code", "datum_code"])?
        .map(|row| {
            let [Some(Field::IntLike(datum_ensemble_code)), Some(Field::IntLike(datum_code))] = row
            else {
                return Err::<_, Box<dyn Error>>(format!("Missing code in {row:?}").into());
            };
            Ok((
                u32::try_from(datum_ensemble_code)?,
                u32::try_from(datum_code)?,
            ))
        })
    {
        let (e, d) = r?;
        datum_ensemble_member_table
            .entry(e)
            .and_modify(|v| v.push(d))
            .or_insert(vec![d]);
    }

    let mut constructors_map = phf_codegen::Map::new();
    let mut ellipsoids_map = phf_codegen::Map::new();
    let mut names_map = phf_codegen::Map::new();
    let mut areas_map = phf_codegen::Map::new();

    for (code, crs) in &crs_table {
        let name = names_table
            .get(code)
            .unwrap_or(&"Unknown Coordinate Reference System");
        let areas = usages_table.get(code);
        match crs {
            CrsEntry::Geographic2D { datum: _ } => {
                constructors_map.entry(code, "&IdentityProjection as &dyn Projection");
                names_map.entry(code, &format!("{name:?}"));
                if let Some(areas) = areas {
                    let mut areas_string = String::new();
                    areas_string.push_str("&[");
                    for (_, [e, n, w, s]) in areas {
                        // TODO make a real type

                        areas_string.push_str(&format!("[{e:?}, {n:?}, {w:?}, {s:?}],"));
                    }
                    areas_string.push_str("]");
                    areas_map.entry(code, &areas_string);
                }
            }
            CrsEntry::Projected { conversion, base } => {
                let Some(CrsEntry::Geographic2D { datum }) = crs_table.get(base) else {
                    //println!("cargo:warning=Skipping EPSG:{code} because base CRS EPSG:{base} does not resolve.");
                    continue;
                };
                let Some((ellipsoid, ellipsoid_code)) = std::iter::once(datum)
                    .chain(
                        datum_ensemble_member_table
                            .get(datum)
                            .iter()
                            .flat_map(|v| v.iter()),
                    )
                    .filter_map(|d| datum_table.get(d))
                    .filter_map(|(e, _)| ellipsoids.get(e).map(|ell| (ell, e))) //this is the spot to handle meridians as well
                    .next()
                else {
                    //println!("cargo:warning=Skipping EPSG:{code} because datum EPSG:{datum} does not resolve.");
                    continue;
                };
                let Some(param_values) = paramvalues.get(conversion) else {
                    //println!("cargo:warning=Skipping EPSG:{code} because parameter values do not resolve.");
                    continue;
                };
                let Some(op_code) = op_table.get(conversion) else {
                    //println!("cargo:warning=Skipping EPSG:{code} because operation EPSG:{conversion} does not resolve.");
                    continue;
                };
                let Some((_, conv)) = supporteds.iter().find(|(v, _)| v == op_code) else {
                    //println!("cargo:warning=Skipping EPSG:{code} because operation method EPSG:{op_code} is not implemented.");
                    continue;
                };
                constructors_map.entry(
                    code,
                    &format!("&{} as &dyn Projection", conv(param_values, *ellipsoid)),
                );
                ellipsoids_map.entry(code, &format!("{ellipsoid_code}"));
                names_map.entry(code, &format!("{name:?}"));
                if let Some(areas) = areas {
                    let mut areas_string = String::new();
                    areas_string.push_str("&[");
                    for (_, [e, n, w, s]) in areas {
                        // TODO make a real type

                        areas_string.push_str(&format!("[{e:?}, {n:?}, {w:?}, {s:?}],"));
                    }
                    areas_string.push_str("]");
                    areas_map.entry(code, &areas_string);
                }
            }
        }
    }

    Ok(format!(
        r"#[allow(clippy::approx_constant)]
static PROJECTIONS: phf::Map<u32, &dyn Projection> = {};
static ELLIPSOIDS: phf::Map<u32, u32> = {};
static NAMES: phf::Map<u32, &str> = {};
static AREAS: phf::Map<u32, &[[f64; 4]]> = {};
",
        constructors_map.build(),
        ellipsoids_map.build(),
        names_map.build(),
        areas_map.build()
    ))
}
