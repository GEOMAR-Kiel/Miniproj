//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use std::collections::HashMap;

use miniproj_ops::ellipsoid::Ellipsoid;
use miniproj_ops::PseudoSerialize;
use rusqlite::{Connection, Result};
use crate::{helpers::*, ImplementedProjection};

/// Generates rust source code mapping EPSG codes to `Ellipsoid`s.
pub fn gen_ellipsoid_constructors(c: &Connection) -> Result<String> {
    let mut s = c.prepare("
        SELECT
            ellipsoid_code as code,
            ellipsoid_name as name,
            semi_major_axis * uom.factor_b / uom.factor_c as a,
            semi_minor_axis * uom.factor_b / uom.factor_c as b,
            inv_flattening as inv_f
        FROM 
            'epsg_ellipsoid' as ellipsoid 
            JOIN 'epsg_unitofmeasure' as uom USING (uom_code);
    ")?;
    let mut constant_defs: String = String::from("static ELLIPSOIDS: phf::Map<u32, Ellipsoid> =");
    let mut phf_map = phf_codegen::Map::new();
    s.query([])?.mapped(|r|
        Ok({
            let code: u32 = r.get_unwrap("code");
            let semi_major = r.get_unwrap::<_, f64>("a");
            let semi_minor = r.get_unwrap::<_, Option<f64>>("b");
            let inf_flat = r.get_unwrap::<_, Option<f64>>("inv_f");
            let ellipsoid = match (semi_minor, inf_flat) {
                (Some(b), _) => {
                    Ellipsoid::from_a_b(semi_major, b)
                },
                (_, Some(f_inv)) => {
                    Ellipsoid::from_a_f_inv(semi_major, f_inv)
                },
                _ => unreachable!("Malformed DB: Ellipsoids need either b or f_inv.")
            };
            phf_map.entry(code, &ellipsoid.to_constructed());
        }))
    .collect::<Result<()>>()?;
    constant_defs.push_str(&phf_map.build().to_string());
    constant_defs.push(';');
    Ok( constant_defs )
}

/// Constructs a `HashMap` mapping EPSG codes to `Ellipsoid`s.
pub fn get_ellipsoids(c: &Connection) -> Result<HashMap<u32, Ellipsoid>> {
    let mut s = c.prepare("
    SELECT
        ellipsoid_code as code,
        ellipsoid_name as name,
        semi_major_axis * uom.factor_b / uom.factor_c as a,
        semi_minor_axis * uom.factor_b / uom.factor_c as b,
        inv_flattening as inv_f
    FROM 
        'epsg_ellipsoid' as ellipsoid 
        JOIN 'epsg_unitofmeasure' as uom USING (uom_code);
    ")?;
    let mut ellipsoids = HashMap::new();
    s.query([])?.mapped(|r|
        Ok({
            let code: u32 = r.get_unwrap("code");
            let semi_major = r.get_unwrap::<_, f64>("a");
            let semi_minor = r.get_unwrap::<_, Option<f64>>("b");
            let inf_flat = r.get_unwrap::<_, Option<f64>>("inv_f");
            ellipsoids.insert(code, match (semi_minor, inf_flat) {
                (Some(b), _) => {
                    Ellipsoid::from_a_b(semi_major, b)
                },
                (_, Some(f_inv)) => {
                    Ellipsoid::from_a_f_inv(semi_major, f_inv)
                },
                _ => unreachable!("Malformed DB: Ellipsoids need either b or f_inv.")
            });
        }))
    .collect::<Result<()>>()?;
    Ok(ellipsoids)
}

/// Generates rust source code mapping EPSG codes to prime meridian angles in radians relative to the Greenwich meridian.
pub fn gen_prime_meridians_source(c: &Connection) -> Result<String> {
    let mut s = c.prepare("
        SELECT
	        prime_meridian_code as code,
	        prime_meridian_name as name,
	        greenwich_longitude * uom.factor_b / uom.factor_c as g_conv,
	        greenwich_longitude as g,
	        uom.uom_code as uom_code
        FROM 
	        'epsg_primemeridian' as prime_meridian 
	        JOIN 'epsg_unitofmeasure' as uom USING (uom_code);
    ")?;
    let mut constant_defs: String = String::from("static PRIME_MERIDIANS: phf::Map<u32, f64> =");
    let mut phf_map = phf_codegen::Map::new();
    s.query([])?.mapped(|r| Ok({
        let code: u32 = r.get_unwrap("code");
        let greenwich_relative = 
            r.get_unwrap::<_, Option<f64>>("g_conv")
            .unwrap_or_else(|| 
                if r.get_unwrap::<_, u32>("uom_code") == 9110 {
                    epsg_9110_to_rad(r.get_unwrap("g"))
                } else {
                    unimplemented!("Meridian relative position in unsupported format.")
            });
        phf_map.entry(code, &format!("f64::from_bits(0x{:x})", greenwich_relative.to_bits()));
    })).collect::<Result<()>>()?;
    constant_defs.push_str(&phf_map.build().to_string());
    constant_defs.push(';');
    Ok(constant_defs)
}


/// Constructs a `HashMap` mapping EPSG codes to prime meridian angles in radians relative to the Greenwich meridian.
pub fn get_prime_meridians(c: &Connection) -> Result<HashMap<u32, f64>> {
    let mut s = c.prepare("
        SELECT
            prime_meridian_code as code,
            prime_meridian_name as name,
            greenwich_longitude * uom.factor_b / uom.factor_c as g_conv,
            greenwich_longitude as g,
            uom.uom_code as uom_code
        FROM 
            'epsg_primemeridian' as prime_meridian 
            JOIN 'epsg_unitofmeasure' as uom USING (uom_code);
    ")?;
    let mut meridians = HashMap::new();
    s.query([])?.mapped(|r| Ok({
        let code: u32 = r.get_unwrap("code");
        let greenwich_relative = 
            r.get_unwrap::<_, Option<f64>>("g_conv")
            .unwrap_or_else(|| 
                if r.get_unwrap::<_, u32>("uom_code") == 9110 {
                    epsg_9110_to_rad(r.get_unwrap("g"))
                } else {
                    unimplemented!("Meridian relative position in unsupported format.")
            });
        meridians.insert(code, greenwich_relative);
    })).collect::<Result<()>>()?;
    Ok(meridians)
}

/// Generates rust source code for projected and geographic coordinate systems for all implemented projections.
pub fn gen_parameter_constructors(c: &Connection, supporteds: &[ImplementedProjection], ellipsoids: &HashMap<u32, Ellipsoid>) -> Result<String> {
    let mut s = c.prepare("
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
    ")?;
    let mut param_value_s = c.prepare("
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
    ")?;
    let mut constant_defs: String = String::from("static PROJECTIONS: phf::Map<u32, &(dyn Projection + Send + Sync)> =");
    let mut constructors_map = phf_codegen::Map::new();
    let mut names_map = phf_codegen::Map::new();
    //Special case for 4326
    constructors_map.entry(4326, "&ZeroProjection as &(dyn Projection + Send + Sync)");
    let mut counter = 1;
    s.query([])?.mapped(|r| Ok({
        let code: u32 = r.get_unwrap("code");
        let name: String = string_to_const_name(&r.get_unwrap::<_, String>("name")) + &format!("_EPSG_{}", code);
        names_map.entry(code, &format!("\"{name}\""));
        let ellipsoid_code: u32 = r.get_unwrap("ellipsoid");
        let _primemerid_code: u32 = r.get_unwrap("primemerid"); //TODO: use correct meridian on exotic projections
        let op_code: u32 = r.get_unwrap("op");
        let method_code: u32 = r.get_unwrap("method");
        let params: Vec<(u32, f64)> = param_value_s.query([op_code])?.mapped(|r| Ok({
            let pcode: u32 = r.get_unwrap("code");
            let pval: f64 = r.get_unwrap::<_, Option<f64>>("v_conv")
            .unwrap_or_else(|| 
                if r.get_unwrap::<_, u32>("uom_code") == 9110 {
                    epsg_9110_to_rad(r.get_unwrap("v"))
                } else {
                    unimplemented!("Parameter in unsupported format.")
            });
            (pcode, pval)
        })).collect::<Result<Vec<_>>>()?;
        let ellipsoid = ellipsoids.get(&ellipsoid_code).expect("Ellipsoid not specified.");
        supporteds.iter().find(|(code, _)| *code == method_code).map(|(_, conv)| {
            constructors_map.entry(code, &format!("&{} as &(dyn Projection + Send + Sync)", conv(&params, *ellipsoid)));
            counter += 1;
        });
    })).collect::<Result<()>>()?;
    println!("Collected {} projected coordinate systems.", counter);
    constant_defs.push_str(&constructors_map.build().to_string());
    constant_defs.push(';');
    constant_defs.push('\n');
    Ok(constant_defs)
}