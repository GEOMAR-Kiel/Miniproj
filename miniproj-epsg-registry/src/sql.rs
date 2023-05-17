use std::collections::HashMap;

use sqlparser::{dialect::GenericDialect, parser::Parser, ast::{ObjectName, SetExpr, Expr, DataType, ColumnOption, Value, UnaryOperator}};

static DB: &'static str = include_str!("../data/gen_reg.sql");

#[derive(Debug)]
pub struct MemoryDb {
    tables: HashMap<String, Table>,
}

#[derive(Debug)]
pub struct Table {
    columns: Vec<Column>,
}
impl Table {
    pub fn get_row_where_i64(&self, col: String, val: i64) -> Option<Vec<Option<Field>>> {
        let Column{name: _, data} = self.columns.iter().find(|Column{name, data: _}| name == &col)?;
        let index = match data {
            ColumnData::IntLike(v) => v.iter().enumerate().find(|(n, v)| **v == val)?.0,
            ColumnData::MaybeIntLike(v) => v.iter().enumerate().find(|(n, v)| v.map(|v| v == val).unwrap_or(false))?.0,
            _ => return None
        };
        Some(self.columns.iter().map(|Column{name: _, data}| {
            match data {
                ColumnData::StringLike(v) => v.get(index).cloned().map(|v| Field::StringLike(v)),
                ColumnData::MaybeStringLike(v) => v.get(index).map(|v| v.as_deref())
                    .flatten().map(|v| Field::StringLike(v.to_owned())),
                ColumnData::IntLike(v) =>  v.get(index).cloned().map(|v| Field::IntLike(v)),
                ColumnData::MaybeIntLike(v) =>  v.get(index).cloned()
                    .flatten().map(|v| Field::IntLike(v.to_owned())),
                ColumnData::Double(v) =>  v.get(index).cloned().map(|v| Field::Double(v)),
                ColumnData::MaybeDouble(v) =>  v.get(index).cloned().flatten().map(|v| Field::Double(v)),
            }
        }).collect())
    }

    pub fn get_rows_where_i64(&self, col: String, val: i64) -> Vec<Vec<Option<Field>>> {
        let Some(Column{name: _, data}) = self.columns.iter().find(|Column{name, data: _}| name == &col) else {return Vec::new()};
        let indices: Vec<_> = match data {
            ColumnData::IntLike(v) => v.iter().enumerate().filter(|(n, v)| **v == val).map(|(i, _)| i).collect(),
            ColumnData::MaybeIntLike(v) => v.iter().enumerate().filter(|(n, v)| v.map(|v| v == val).unwrap_or(false)).map(|(i, _)| i).collect(),
            _ => return Vec::new()
        };
        indices.into_iter().map(|index| 
            self.columns.iter().map(|Column{name: _, data}| {
            match data {
                ColumnData::StringLike(v) => v.get(index).cloned().map(|v| Field::StringLike(v)),
                ColumnData::MaybeStringLike(v) => v.get(index).map(|v| v.as_deref())
                    .flatten().map(|v| Field::StringLike(v.to_owned())),
                ColumnData::IntLike(v) =>  v.get(index).cloned().map(|v| Field::IntLike(v)),
                ColumnData::MaybeIntLike(v) =>  v.get(index).cloned()
                    .flatten().map(|v| Field::IntLike(v.to_owned())),
                ColumnData::Double(v) =>  v.get(index).cloned().map(|v| Field::Double(v)),
                ColumnData::MaybeDouble(v) =>  v.get(index).cloned().flatten().map(|v| Field::Double(v)),
            }
        }).collect()).collect()
    }
}

#[derive(Debug)]
pub struct Column{
    name: String,
    data: ColumnData
}

pub enum ColumnData {
    StringLike(Vec<String>),
    MaybeStringLike(Vec<Option<String>>),
    IntLike(Vec<i64>),
    MaybeIntLike(Vec<Option<i64>>),
    Double(Vec<f64>),
    MaybeDouble(Vec<Option<f64>>)
}

pub enum Field {
    StringLike(String),
    IntLike(i64),
    Double(f64)
}

impl std::fmt::Debug for ColumnData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringLike(arg0) => f.debug_tuple("StringLike").field(&arg0.len()).finish(),
            Self::MaybeStringLike(arg0) => f.debug_tuple("MaybeStringLike").field(&arg0.len()).finish(),
            Self::IntLike(arg0) => f.debug_tuple("IntLike").field(&arg0.len()).finish(),
            Self::MaybeIntLike(arg0) => f.debug_tuple("MaybeIntLike").field(&arg0.len()).finish(),
            Self::Double(arg0) => f.debug_tuple("Double").field(&arg0.len()).finish(),
            Self::MaybeDouble(arg0) => f.debug_tuple("MaybeDouble").field(&arg0.len()).finish(),
        }
    }
}

impl MemoryDb {
    pub fn new() -> Self {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, DB).expect("Parser error.");
        let mut tables = HashMap::new();
        for stmt in &ast {
            match stmt {
                a @ sqlparser::ast::Statement::Insert {
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
                } => {
                    let table: &mut Table = tables.get_mut(&table_name.0.iter().last().unwrap().value).unwrap();
                    let SetExpr::Values(values) = source.body.as_ref() else {panic!("expected values!")};
                    for row in &values.rows {
                        assert_eq!(table.columns.len(), row.len());
                        for (expr, Column{name, data}) in row.iter().zip(table.columns.iter_mut()) {
                            match (data, expr) {
                                (ColumnData::MaybeStringLike(v), Expr::Value(Value::Null)) => v.push(None),
                                (ColumnData::MaybeIntLike(v), Expr::Value(Value::Null)) => v.push(None),
                                (ColumnData::MaybeDouble(v), Expr::Value(Value::Null)) => v.push(None),
                                (ColumnData::IntLike(v), Expr::Value(Value::Number(n, _))) => v.push(n.parse().expect("cannot parse i64")),
                                (ColumnData::MaybeIntLike(v), Expr::Value(Value::Number(n, _))) => v.push(Some(n.parse().expect("cannot parse i64"))),
                                (ColumnData::StringLike(v), Expr::Value(Value::SingleQuotedString(s))) => v.push(s.clone()),
                                (ColumnData::MaybeStringLike(v), Expr::Value(Value::SingleQuotedString(s))) => v.push(Some(s.clone())),
                                (ColumnData::Double(v), Expr::Value(Value::Number(n, _))) => v.push(n.parse().expect("cannot parse f64")),
                                (ColumnData::Double(v), Expr::UnaryOp{op:UnaryOperator::Minus, expr}) => {
                                    let Expr::Value(Value::Number(n, _)) = expr.as_ref() else {panic!("cannot negate non-numbers")};
                                    v.push(n.parse().expect("cannot parse f64"));
                                },
                                (ColumnData::MaybeDouble(v), Expr::Value(Value::Number(n, _))) => v.push(Some(n.parse().expect("cannot parse f64"))),
                                (ColumnData::MaybeDouble(v), Expr::UnaryOp{op:UnaryOperator::Minus, expr}) => {
                                    let Expr::Value(Value::Number(n, _)) = expr.as_ref() else {panic!("cannot negate non-numbers")};
                                    v.push(Some(n.parse().expect("cannot parse f64")));
                                },
                                (d, e) => panic!("cannot push {:?} to {:?}.", e, d)
                            }
                        }
                    }
                },
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
                } => {
                    tables.insert(
                        name.0.last().unwrap().value.clone(), 
                        Table { columns: 
                            columns.into_iter().map(|c| 
                                Column {
                                    name: c.name.value.clone(),
                                    data: if c.options.iter().any(|o| o.option == ColumnOption::NotNull) {
                                        match &c.data_type {
                                            DataType::Real | DataType::Double | DataType::DoublePrecision | DataType::Float(_) => ColumnData::Double(Vec::new()),
                                            DataType::Integer(_) | DataType::SmallInt(_) => ColumnData::IntLike(Vec::new()),
                                            DataType::Varchar(_) | DataType::Date => ColumnData::StringLike(Vec::new()),
                                            a => panic!("type {:?} not supported!", a),
                                        }
                                    } else {
                                        match &c.data_type {
                                            DataType::Real | DataType::Double | DataType::DoublePrecision | DataType::Float(_) => ColumnData::MaybeDouble(Vec::new()),
                                            DataType::Varchar(_) | DataType::Date => ColumnData::MaybeStringLike(Vec::new()),
                                            DataType::Integer(_) | DataType::SmallInt(_) | DataType::Custom(_, _) => ColumnData::MaybeIntLike(Vec::new()),
                                            a => panic!("type {:?} not supported!", a),
                                        }
                                    }
                                }
                            ).collect(),
                        }
                    );
                },
                _ => {}
            }
        }
        Self {
            tables,
        }
    }
}
