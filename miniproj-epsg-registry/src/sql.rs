use std::collections::HashMap;

use sqlparser::{
    ast::{ColumnOption, DataType, Expr, SetExpr, UnaryOperator, Value},
    dialect::GenericDialect,
    parser::Parser,
};

static DB: &'static str = include_str!("../data/gen_reg.sql");

#[derive(Debug)]
pub struct MemoryDb {
    tables: HashMap<String, Table>,
}
impl MemoryDb {
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}

#[derive(Debug)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    pub fn rows(&self) -> Option<usize> {
        self.columns.first().map(Column::len)
    }

    pub fn get_row_where_i64<const N: usize>(&self, col: String, val: i64, select: [&str; N]) -> Option<[Option<Field>; N]> {
        if self.columns.len() != N {return None}
        let Column { name: _, data } = self
            .columns
            .iter()
            .find(|Column { name, data: _ }| name == &col)?;
        let index = match data {
            ColumnData::IntLike(v) => v.iter().enumerate().find(|(_n, v)| **v == val)?.0,
            ColumnData::MaybeIntLike(v) => {
                v.iter()
                    .enumerate()
                    .find(|(_n, v)| v.map(|v| v == val).unwrap_or(false))?
                    .0
            }
            _ => return None,
        };
        let mut res = [None; N];
            select
                .iter()
                .zip(res.iter_mut())
                .try_for_each(|(select_name, field)| {
                        let Column { name: _, data } = self.columns.iter().find(|Column { name, data: _ }| name == select_name)?;
                        *field = match data {
                            ColumnData::StringLike(v) => {
                                v.get(index).map(|v| Field::StringLike(v))
                            }
                            ColumnData::MaybeStringLike(v) => v
                                .get(index)
                                .map(|v| v.as_deref())
                                .flatten()
                                .map(|v| Field::StringLike(v)),
                            ColumnData::IntLike(v) => v.get(index).cloned().map(|v| Field::IntLike(v)),
                            ColumnData::MaybeIntLike(v) => v
                                .get(index)
                                .cloned()
                                .flatten()
                                .map(|v| Field::IntLike(v.to_owned())),
                            ColumnData::Double(v) => v.get(index).cloned().map(|v| Field::Double(v)),
                            ColumnData::MaybeDouble(v) => {
                                v.get(index).cloned().flatten().map(|v| Field::Double(v))
                            }
                        };
                        Some(())
                    }
                );
        Some(res)
    }

    pub fn get_rows<const N: usize>(&self, select: [&str; N]) -> Vec<[Option<Field>; N]> {
        if self.columns.len() != N {return Vec::new()}
        let Some(len) = self.rows() else{return Vec::new()};
        (0..len).map(|index| {
                let mut tmp = [None; N];
                self.columns
                    .iter()
                    .zip(tmp.iter_mut())
                    .for_each(|(Column { name: _, data }, field)| *field = match data {
                        ColumnData::StringLike(v) => {
                            v.get(index).map(|v| Field::StringLike(v))
                        }
                        ColumnData::MaybeStringLike(v) => v
                            .get(index)
                            .map(|v| v.as_deref())
                            .flatten()
                            .map(|v| Field::StringLike(v)),
                        ColumnData::IntLike(v) => v.get(index).cloned().map(|v| Field::IntLike(v)),
                        ColumnData::MaybeIntLike(v) => v
                            .get(index)
                            .cloned()
                            .flatten()
                            .map(|v| Field::IntLike(v.to_owned())),
                        ColumnData::Double(v) => v.get(index).cloned().map(|v| Field::Double(v)),
                        ColumnData::MaybeDouble(v) => {
                            v.get(index).cloned().flatten().map(|v| Field::Double(v))
                        }
                    });
                tmp
            })
            .collect()
    }

    pub fn get_rows_where_i64<const N: usize>(&self, col: &str, val: i64) -> Vec<[Option<Field>; N]> {
        if self.columns.len() != N {return Vec::new()}
        let Some(Column{name: _, data}) = self.columns.iter().find(|Column{name, data: _}| name == &col) else {return Vec::new()};
        let indices: Vec<_> = match data {
            ColumnData::IntLike(v) => v
                .iter()
                .enumerate()
                .filter(|(_n, v)| **v == val)
                .map(|(i, _)| i)
                .collect(),
            ColumnData::MaybeIntLike(v) => v
                .iter()
                .enumerate()
                .filter(|(_n, v)| v.map(|v| v == val).unwrap_or(false))
                .map(|(i, _)| i)
                .collect(),
            _ => return Vec::new(),
        };
        indices
            .into_iter()
            .map(|index| {
                let mut tmp = [None; N];
                self.columns
                    .iter()
                    .zip(tmp.iter_mut())
                    .for_each(|(Column { name: _, data }, field)| *field = match data {
                        ColumnData::StringLike(v) => {
                            v.get(index).map(|v| Field::StringLike(v))
                        }
                        ColumnData::MaybeStringLike(v) => v
                            .get(index)
                            .map(|v| v.as_deref())
                            .flatten()
                            .map(|v| Field::StringLike(v)),
                        ColumnData::IntLike(v) => v.get(index).cloned().map(|v| Field::IntLike(v)),
                        ColumnData::MaybeIntLike(v) => v
                            .get(index)
                            .cloned()
                            .flatten()
                            .map(|v| Field::IntLike(v.to_owned())),
                        ColumnData::Double(v) => v.get(index).cloned().map(|v| Field::Double(v)),
                        ColumnData::MaybeDouble(v) => {
                            v.get(index).cloned().flatten().map(|v| Field::Double(v))
                        }
                    });
                tmp
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    data: ColumnData,
}
impl Column {
    pub fn len(&self) -> usize {
        match &self.data {
            ColumnData::StringLike(v) => v.len(),
            ColumnData::MaybeStringLike(v) => v.len(),
            ColumnData::IntLike(v) => v.len(),
            ColumnData::MaybeIntLike(v) => v.len(),
            ColumnData::Double(v) => v.len(),
            ColumnData::MaybeDouble(v) => v.len(),
        }
    }
}

pub enum ColumnData {
    StringLike(Vec<String>),
    MaybeStringLike(Vec<Option<String>>),
    IntLike(Vec<i64>),
    MaybeIntLike(Vec<Option<i64>>),
    Double(Vec<f64>),
    MaybeDouble(Vec<Option<f64>>),
}

#[derive(Copy, Clone, Debug)]
pub enum Field<'s> {
    StringLike(&'s str),
    IntLike(i64),
    Double(f64),
}

impl std::fmt::Debug for ColumnData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringLike(arg0) => f.debug_tuple("StringLike").field(&arg0.len()).finish(),
            Self::MaybeStringLike(arg0) => {
                f.debug_tuple("MaybeStringLike").field(&arg0.len()).finish()
            }
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
                _a @ sqlparser::ast::Statement::Insert {
                    or: _,
                    into: _,
                    table_name,
                    columns: _,
                    overwrite: _,
                    source,
                    partitioned: _,
                    after_columns: _,
                    table: _,
                    on: _,
                    returning: _,
                } => {
                    let table: &mut Table = tables
                        .get_mut(&table_name.0.iter().last().unwrap().value)
                        .unwrap();
                    let SetExpr::Values(values) = source.body.as_ref() else {panic!("expected values!")};
                    for row in &values.rows {
                        assert_eq!(table.columns.len(), row.len());
                        for (expr, Column { name: _, data }) in
                            row.iter().zip(table.columns.iter_mut())
                        {
                            match (data, expr) {
                                (ColumnData::MaybeStringLike(v), Expr::Value(Value::Null)) => {
                                    v.push(None)
                                }
                                (ColumnData::MaybeIntLike(v), Expr::Value(Value::Null)) => {
                                    v.push(None)
                                }
                                (ColumnData::MaybeDouble(v), Expr::Value(Value::Null)) => {
                                    v.push(None)
                                }
                                (ColumnData::IntLike(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(n.parse().expect("cannot parse i64"))
                                }
                                (ColumnData::MaybeIntLike(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(Some(n.parse().expect("cannot parse i64")))
                                }
                                (
                                    ColumnData::StringLike(v),
                                    Expr::Value(Value::SingleQuotedString(s)),
                                ) => v.push(s.clone()),
                                (
                                    ColumnData::MaybeStringLike(v),
                                    Expr::Value(Value::SingleQuotedString(s)),
                                ) => v.push(Some(s.clone())),
                                (ColumnData::Double(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(n.parse().expect("cannot parse f64"))
                                }
                                (
                                    ColumnData::Double(v),
                                    Expr::UnaryOp {
                                        op: UnaryOperator::Minus,
                                        expr,
                                    },
                                ) => {
                                    let Expr::Value(Value::Number(n, _)) = expr.as_ref() else {panic!("cannot negate non-numbers")};
                                    v.push(n.parse().expect("cannot parse f64"));
                                }
                                (ColumnData::MaybeDouble(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(Some(n.parse().expect("cannot parse f64")))
                                }
                                (
                                    ColumnData::MaybeDouble(v),
                                    Expr::UnaryOp {
                                        op: UnaryOperator::Minus,
                                        expr,
                                    },
                                ) => {
                                    let Expr::Value(Value::Number(n, _)) = expr.as_ref() else {panic!("cannot negate non-numbers")};
                                    v.push(Some(n.parse().expect("cannot parse f64")));
                                }
                                (d, e) => panic!("cannot push {:?} to {:?}.", e, d),
                            }
                        }
                    }
                }
                sqlparser::ast::Statement::CreateTable {
                    or_replace: _,
                    temporary: _,
                    external: _,
                    global: _,
                    if_not_exists: _,
                    transient: _,
                    name,
                    columns,
                    constraints: _,
                    hive_distribution: _,
                    hive_formats: _,
                    table_properties: _,
                    with_options: _,
                    file_format: _,
                    location: _,
                    query: _,
                    without_rowid: _,
                    like: _,
                    clone: _,
                    engine: _,
                    default_charset: _,
                    collation: _,
                    on_commit: _,
                    on_cluster: _,
                    order_by: _,
                } => {
                    tables.insert(
                        name.0.last().unwrap().value.clone(),
                        Table {
                            columns: columns
                                .into_iter()
                                .map(|c| Column {
                                    name: c.name.value.clone(),
                                    data: if c
                                        .options
                                        .iter()
                                        .any(|o| o.option == ColumnOption::NotNull)
                                    {
                                        match &c.data_type {
                                            DataType::Real
                                            | DataType::Double
                                            | DataType::DoublePrecision
                                            | DataType::Float(_) => ColumnData::Double(Vec::new()),
                                            DataType::Integer(_) | DataType::SmallInt(_) => {
                                                ColumnData::IntLike(Vec::new())
                                            }
                                            DataType::Varchar(_) | DataType::Date => {
                                                ColumnData::StringLike(Vec::new())
                                            }
                                            a => panic!("type {:?} not supported!", a),
                                        }
                                    } else {
                                        match &c.data_type {
                                            DataType::Real
                                            | DataType::Double
                                            | DataType::DoublePrecision
                                            | DataType::Float(_) => {
                                                ColumnData::MaybeDouble(Vec::new())
                                            }
                                            DataType::Varchar(_) | DataType::Date => {
                                                ColumnData::MaybeStringLike(Vec::new())
                                            }
                                            DataType::Integer(_)
                                            | DataType::SmallInt(_)
                                            | DataType::Custom(_, _) => {
                                                ColumnData::MaybeIntLike(Vec::new())
                                            }
                                            a => panic!("type {:?} not supported!", a),
                                        }
                                    },
                                })
                                .collect(),
                        },
                    );
                }
                _ => {}
            }
        }
        Self { tables }
    }
}
