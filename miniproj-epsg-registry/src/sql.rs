use std::collections::HashMap;

use sqlparser::{dialect::GenericDialect, parser::Parser};

static DB: &'static str = include_str!("../data/gen_reg.sql");

pub struct MemoryDb {
    tables: HashMap<String, Table>,
}

pub struct Table {
    fields: Vec<String>,
    data: Vec<Vec<String>>,
}

impl MemoryDb {
    pub fn new() -> Self {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, DB).expect("Parser error.");
        for stmt in &ast {
            match stmt {
                sqlparser::ast::Statement::Insert {
                    or,
                    into,
                    table_name,
                    columns,
                    overwrite,
                    source,
                    partitioned,
                    after_columns,
                    table,
                    on,
                    returning,
                } => todo!(),
                sqlparser::ast::Statement::CreateTable {
                    or_replace,
                    temporary,
                    external,
                    global,
                    if_not_exists,
                    transient,
                    name,
                    columns,
                    constraints,
                    hive_distribution,
                    hive_formats,
                    table_properties,
                    with_options,
                    file_format,
                    location,
                    query,
                    without_rowid,
                    like,
                    clone,
                    engine,
                    default_charset,
                    collation,
                    on_commit,
                    on_cluster,
                    order_by,
                } => todo!(),
                _ => {}
            }
        }
        Self {
            tables: HashMap::new(),
        }
    }
}
