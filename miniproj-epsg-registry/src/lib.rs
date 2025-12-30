//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

mod db;
mod helpers;
mod sql;

pub use crate::db::*;
pub use crate::sql::*;
use miniproj_ops::ellipsoid::Ellipsoid;

#[cfg(test)]
mod tests {
    use crate::sql::MemoryDb;

    #[test]
    fn create_mem_db() {
        let memdb = MemoryDb::new();
        eprintln!("{memdb:#?}")
    }
}
