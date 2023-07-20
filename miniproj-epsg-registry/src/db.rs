//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::{collections::HashMap, error::Error, hash::Hash, num::TryFromIntError};

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
        let [Some(Field::IntLike(code)), _, _, _, Some(Field::IntLike(uom_code))] = a else {unreachable!("No UOM Code given. (row: {:?})", a)};
        let Some([_ , Some(Field::Double(fac_b)), Some(Field::Double(fac_c))]) = uom_rows.iter().find(|[f, _, _]| {
            if let Some(Field::IntLike(code)) = f  {
                code == uom_code
            } else {
                false
            }
        }) else {unreachable!("No UOM found for Code in DB.")};
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
        let [Some(Field::IntLike(code)), _, _, _, Some(Field::IntLike(uom_code))] = a else {return Err(format!("No UOM Code given. (row: {:?})", a).into())};
        let Some([_ , Some(Field::Double(fac_b)), Some(Field::Double(fac_c))]) = uom_rows.iter().find(|[f, _, _]| {
            if let Some(Field::IntLike(code)) = f  {
                code == uom_code
            } else {
                false
            }
        }) else {unreachable!("No UOM found for Code in DB.")};
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
pub fn gen_prime_meridians_source(c: &MemoryDb) -> Result<String, Box<dyn Error>> {
    todo!()
}

/// Constructs a `HashMap` mapping EPSG codes to prime meridian angles in radians relative to the Greenwich meridian.
pub fn get_prime_meridians(c: &MemoryDb) -> Result<HashMap<u32, f64>, Box<dyn Error>> {
    todo!()
}

#[derive(Debug)]
enum CrsEntry {
    Geographic2D{datum: u32},
    Projected{conversion: u32, base: u32}
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
                _ => None
            }
        })
        .collect::<HashMap<u32, _>>();
    assert!(!crs_table.is_empty());
    let op_table = db.get_table("epsg_coordoperation")
        .ok_or("No Op table")?
        .get_rows(&["coord_op_code", "coord_op_method_code"])?
        .filter_map(|row| {
            let [Some(Field::IntLike(coord_op_code)), Some(Field::IntLike(coord_op_method_code))] = row else {return None};
            match (u32::try_from(coord_op_code), u32::try_from(coord_op_method_code)) {
                (Ok(coord_op_code), Ok(coord_op_method_code)) => {
                    Some(Ok((coord_op_code, coord_op_method_code)))
                }
                (Err(e), _) | (_, Err(e)) => Some(Err(e))
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
    for r in db.get_table("epsg_datumensemblemember")
        .ok_or("No Datum Ensemble Member table")?
        .get_rows(&["datum_ensemble_code", "datum_code"])?
        .map(|row| {
            let [Some(Field::IntLike(datum_ensemble_code)), Some(Field::IntLike(datum_code))] = row else {return Err::<_, Box<dyn Error>>(format!("Missing code in {row:?}").into())};
            Ok((u32::try_from(datum_ensemble_code)?, u32::try_from(datum_code)?))
        }) {
            let (e, d) = r?;
            datum_ensemble_member_table.entry(e).and_modify(|v| v.push(d)).or_insert(vec![d]);
        }

    let mut constructors_map = phf_codegen::Map::new();
    let mut ellipsoids_map = phf_codegen::Map::new();

    for (code, crs) in &crs_table {
        match crs {
            CrsEntry::Geographic2D { datum: _ } => {
                constructors_map.entry(code, "&ZeroProjection as &(dyn Projection + Send + Sync)");
            },
            CrsEntry::Projected { conversion, base } => {
                let Some(CrsEntry::Geographic2D { datum }) = crs_table.get(base) else {
                    //println!("cargo:warning=Skipping EPSG:{code} because base CRS EPSG:{base} does not resolve.");
                    continue;
                };
                let Some((ellipsoid, ellipsoid_code)) = std::iter::once(datum)
                    .chain(datum_ensemble_member_table.get(datum).iter().flat_map(|v| v.iter()))
                    .filter_map(|d| datum_table.get(d))
                    .filter_map(|(e, _)| ellipsoids.get(e).map(|ell| (ell, e))) //this is the spot to handle meridians as well
                    .next() else {
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
                    &format!(
                        "&{} as &(dyn Projection + Send + Sync)",
                        conv(param_values, *ellipsoid)
                    ),
                );
                ellipsoids_map.entry(code, &format!("{ellipsoid_code}"));
            },
        }
    }

    Ok(format!(
        r"#[allow(clippy::approx_constant)]
static PROJECTIONS: phf::Map<u32, &(dyn Projection + Send + Sync)> = {};
static ELLIPSOIDS: phf::Map<u32, u32> = {};
",
        constructors_map.build(),
        ellipsoids_map.build()
    ))

    /*
        let mut s = c.prepare(
            "
            SELECT
                crs.coord_ref_sys_code AS code,
                crs.coord_ref_sys_name AS name,
                datum.ellipsoid_code AS ellipsoid,
                datum.prime_meridian_code AS primemerid,
                operation.coord_op_code AS op,
                operation.coord_op_method_code AS method
            FROM
                epsg_coordinatereferencesystem as crs
                JOIN epsg_coordinatereferencesystem as base_crs
                    ON crs.base_crs_code = base_crs.coord_ref_sys_code
                JOIN epsg_coordoperation as operation
                    ON operation.coord_op_code = crs.projection_conv_code
                JOIN (
                    SELECT DISTINCT
                        datum_code,
                        ellipsoid_code,
                        prime_meridian_code
                    FROM
                        epsg_datum
                    WHERE
                        ellipsoid_code IS NOT NULL AND
                        prime_meridian_code IS NOT NULL
                    UNION SELECT DISTINCT
                        datum_ensemble_code as datum_code,
                        ellipsoid_code,
                        prime_meridian_code
                    FROM
                        epsg_datumensemblemember AS ensemble
                        JOIN epsg_datum
                            ON epsg_datum.datum_code = ensemble.datum_code
                    WHERE
                        epsg_datum.ellipsoid_code IS NOT NULL AND
                        epsg_datum.prime_meridian_code IS NOT NULL
                ) AS datum
                    ON datum.datum_code = base_crs.datum_code
            WHERE
                crs.coord_ref_sys_kind = 'projected' AND
                base_crs.coord_ref_sys_kind = 'geographic 2D'
        ",
        )?;
        let mut param_value_s = c.prepare(
            "
            SELECT
                val.parameter_code as code,
                val.parameter_value * uom.factor_b / uom.factor_c as v_conv,
                val.parameter_value as v,
                uom.uom_code as uom_code
            FROM
                'epsg_coordoperationparamvalue' as val
                JOIN 'epsg_unitofmeasure' as uom
                    USING (uom_code)
            WHERE
                val.coord_op_code = (?)
        ",
        )?;
        let mut constructors_map = phf_codegen::Map::new();
        let mut ellipsoids_map = phf_codegen::Map::new();
        let mut names_map = phf_codegen::Map::new();
        //Special case for 4326
        constructors_map.entry(4326, "&ZeroProjection as &(dyn Projection + Send + Sync)");
        let mut counter = 1;
        s.query([])?
            .mapped(|r| {
                {
                    let code: u32 = r.get_unwrap("code");
                    let name: String = string_to_const_name(&r.get_unwrap::<_, String>("name"))
                        + &format!("_EPSG_{}", code);
                    names_map.entry(code, &format!("\"{name}\""));
                    let ellipsoid_code: u32 = r.get_unwrap("ellipsoid");
                    let _primemerid_code: u32 = r.get_unwrap("primemerid"); //TODO: use correct meridian on exotic projections
                    let op_code: u32 = r.get_unwrap("op");
                    let method_code: u32 = r.get_unwrap("method");
                    let params: Vec<(u32, f64)> = param_value_s
                        .query([op_code])?
                        .mapped(|r| {
                            Ok({
                                let pcode: u32 = r.get_unwrap("code");
                                let pval: f64 =
                                    r.get_unwrap::<_, Option<f64>>("v_conv").unwrap_or_else(|| {
                                        if r.get_unwrap::<_, u32>("uom_code") == 9110 {
                                            epsg_9110_to_rad(r.get_unwrap("v"))
                                        } else {
                                            unimplemented!("Parameter in unsupported format.")
                                        }
                                    });
                                (pcode, pval)
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let ellipsoid = ellipsoids
                        .get(&ellipsoid_code)
                        .expect("Ellipsoid not specified.");
                    if let Some((_, conv)) = supporteds.iter().find(|(code, _)| *code == method_code) {
                        constructors_map.entry(
                            code,
                            &format!(
                                "&{} as &(dyn Projection + Send + Sync)",
                                conv(&params, *ellipsoid)
                            ),
                        );
                        ellipsoids_map.entry(code, &format!("{ellipsoid_code}"));
                        counter += 1;
                    }
                };
                Ok(())
            })
            .collect::<Result<()>>()?;
        println!("Collected {} projected coordinate systems.", counter);
        Ok(format!(
            r"#[allow(clippy::approx_constant)]
    static PROJECTIONS: phf::Map<u32, &(dyn Projection + Send + Sync)> = {};
    static ELLIPSOIDS: phf::Map<u32, u32> = {};
    ",
            constructors_map.build(),
            ellipsoids_map.build()
        ))*/
}
