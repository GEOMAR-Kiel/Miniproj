//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

mod db;
mod helpers;
use lazy_static::lazy_static;
pub use crate::db::*;
mod types;
use types::ImplementedConversion;
pub use rusqlite::Connection as DbConnection;



lazy_static! {
    pub static ref IMPL_CONV: Vec<ImplementedConversion> = vec![
        ImplementedConversion::new(
            9807,
            // lon lat k e n
            &[8802, 8801, 8805, 8806, 8807],
            "TransverseMercatorParams",
            "TransverseMercatorConversion"
        ),
        ImplementedConversion::new(
            9820,
            &[8802, 8801, 8806, 8807],
            "LambertAzimuthalEqualAreaParams",
            "LambertAzimuthalEqualAreaConversion"
        )
    ];
}


#[cfg(test)]
mod tests {

}
