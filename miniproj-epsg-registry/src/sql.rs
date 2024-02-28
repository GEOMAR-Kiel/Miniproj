use std::{collections::HashMap, error::Error};

use sqlparser::{
    ast::{ColumnOption, DataType, Expr, SetExpr, UnaryOperator, Value, Ident},
    dialect::GenericDialect,
    parser::Parser,
};

static DB: &str = include_str!("../data/gen_reg.sql");

#[derive(Debug, Default)]
pub struct MemoryDb {
    tables: HashMap<String, Table>,
}
impl MemoryDb {
    #[must_use]
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}

#[derive(Debug)]
pub struct Table {
    column_order: Vec<String>,
    columns: HashMap<String, Column>,
}

impl Table {
    pub fn rows(&self) -> Option<usize> {
        self.columns.values().next().map(Column::len)
    }

    #[must_use]
    pub fn get_row_where_i64<const N: usize>(
        &self,
        col: &str,
        val: i64,
        select: &[&str; N],
    ) -> Option<[Option<Field>; N]> {
        let Column { data } = self.columns.get(col)?;
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
                let Column { data } = self.columns.get(*select_name)?;
                *field = match data {
                    ColumnData::StringLike(v) => v.get(index).map(|v| Field::StringLike(v)),
                    ColumnData::MaybeStringLike(v) => v
                        .get(index)
                        .and_then(std::option::Option::as_deref)
                        .map(Field::StringLike),
                    ColumnData::IntLike(v) => v.get(index).copied().map(Field::IntLike),
                    ColumnData::MaybeIntLike(v) => {
                        v.get(index).copied().flatten().map(Field::IntLike)
                    }
                    ColumnData::Double(v) => v.get(index).copied().map(Field::Double),
                    ColumnData::MaybeDouble(v) => {
                        v.get(index).copied().flatten().map(Field::Double)
                    }
                };
                Some(())
            });
        Some(res)
    }

    pub fn get_rows<const N: usize>(
        &self,
        select: &[&str; N],
    ) -> Result<impl Iterator<Item = [Option<Field>; N]>, Box<dyn Error>> {
        let Some(columns) = select
            .iter()
            .map(|n| self.columns.get(*n))
            .collect::<Option<Vec<_>>>()
        else {
            return Err(format!(
                "could not satisfy cols: {select:?} with {:?}",
                self.column_order
            )
            .into());
        };
        let len = self.rows().unwrap_or(0);
        Ok((0..len).map(move |index| {
            let mut tmp = [None; N];
            columns
                .iter()
                .zip(tmp.iter_mut())
                .for_each(|(Column { data }, field)| {
                    *field = match data {
                        ColumnData::StringLike(v) => v.get(index).map(|v| Field::StringLike(v)),
                        ColumnData::MaybeStringLike(v) => v
                            .get(index)
                            .and_then(std::option::Option::as_deref)
                            .map(Field::StringLike),
                        ColumnData::IntLike(v) => v.get(index).copied().map(Field::IntLike),
                        ColumnData::MaybeIntLike(v) => {
                            v.get(index).copied().flatten().map(Field::IntLike)
                        }
                        ColumnData::Double(v) => v.get(index).copied().map(Field::Double),
                        ColumnData::MaybeDouble(v) => {
                            v.get(index).copied().flatten().map(Field::Double)
                        }
                    }
                });
            tmp
        }))
    }

    #[must_use]
    pub fn get_rows_where_i64<const N: usize>(
        &self,
        col: &str,
        val: i64,
        select: &[&str; N],
    ) -> Vec<[Option<Field>; N]> {
        let Some(columns) = select
            .iter()
            .map(|n| self.columns.get(*n))
            .collect::<Option<Vec<_>>>()
        else {
            return Vec::new();
        };
        let Some(Column { data }) = self.columns.get(col) else {
            return Vec::new();
        };
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
                columns
                    .iter()
                    .zip(tmp.iter_mut())
                    .for_each(|(Column { data }, field)| {
                        *field = match data {
                            ColumnData::StringLike(v) => v.get(index).map(|v| Field::StringLike(v)),
                            ColumnData::MaybeStringLike(v) => v
                                .get(index)
                                .and_then(std::option::Option::as_deref)
                                .map(Field::StringLike),
                            ColumnData::IntLike(v) => v.get(index).copied().map(Field::IntLike),
                            ColumnData::MaybeIntLike(v) => {
                                v.get(index).copied().flatten().map(Field::IntLike)
                            }
                            ColumnData::Double(v) => v.get(index).copied().map(Field::Double),
                            ColumnData::MaybeDouble(v) => {
                                v.get(index).copied().flatten().map(Field::Double)
                            }
                        }
                    });
                tmp
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Column {
    data: ColumnData,
}
impl Column {
    #[must_use]
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

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn new() -> Self {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, DB).expect("Parser error.");
        let mut tables = HashMap::new();
        for stmt in &ast {
            match stmt {
                sqlparser::ast::Statement::Insert {
                    into: true,
                    table_name,
                    source: Some(source),
                    columns,
                    ..
                } => {
                    let table: &mut Table = tables
                        .get_mut(&table_name.0.iter().last().unwrap().value)
                        .unwrap();
                    let SetExpr::Values(ref values) = *source.body else {
                        panic!("expected values!")
                    };
                    for row in &values.rows {
                        
                        let mapping = if columns.is_empty() {
                            if row.len() == table.columns.len() {
                                row.iter().zip(table.column_order.iter()).collect::<Vec<_>>()
                            } else {
                                panic!("table {table_name:#?} could not be set.")
                            }
                        } else {
                            table.column_order.iter().map(|name| {
                                if let Some((index, _)) = columns.iter().enumerate().find(|(_, Ident{value, ..})| {
                                    value == name
                                }) {
                                    (&row[index], name)
                                } else {
                                    (&Expr::Value(Value::Null), name)
                                }
                            }).collect::<Vec<_>>()
                        };
                        
                        for (expr, col_name) in mapping {
                            let Column { data } =
                                table.columns.get_mut(col_name).expect("Missing column.");
                            match (data, expr) {
                                (ColumnData::MaybeStringLike(v), Expr::Value(Value::Null)) => {
                                    v.push(None);
                                }
                                (ColumnData::MaybeIntLike(v), Expr::Value(Value::Null)) => {
                                    v.push(None);
                                }
                                (ColumnData::MaybeDouble(v), Expr::Value(Value::Null)) => {
                                    v.push(None);
                                }
                                (ColumnData::IntLike(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(n.parse().expect("cannot parse i64"));
                                }
                                (ColumnData::MaybeIntLike(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(Some(n.parse().expect("cannot parse i64")));
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
                                    v.push(n.parse().expect("cannot parse f64"));
                                }
                                (
                                    ColumnData::Double(v),
                                    Expr::UnaryOp {
                                        op: UnaryOperator::Minus,
                                        expr,
                                    },
                                ) => {
                                    let Expr::Value(Value::Number(n, _)) = expr.as_ref() else {
                                        panic!("cannot negate non-numbers")
                                    };
                                    v.push(-n.parse::<f64>().expect("cannot parse f64"));
                                }
                                (ColumnData::MaybeDouble(v), Expr::Value(Value::Number(n, _))) => {
                                    v.push(Some(n.parse::<f64>().expect("cannot parse f64")));
                                }
                                (
                                    ColumnData::MaybeDouble(v),
                                    Expr::UnaryOp {
                                        op: UnaryOperator::Minus,
                                        expr,
                                    },
                                ) => {
                                    let Expr::Value(Value::Number(n, _)) = expr.as_ref() else {
                                        panic!("cannot negate non-numbers")
                                    };
                                    v.push(Some(-n.parse::<f64>().expect("cannot parse f64")));
                                }
                                (d, e) => {
                                    panic!("cannot push {e:?} to {d:?}.")
                                }
                            }
                        }
                    }
                }
                sqlparser::ast::Statement::CreateTable { name, columns, .. } => {
                    tables.insert(
                        name.0.last().unwrap().value.clone(),
                        Table {
                            column_order: columns.iter().map(|c| c.name.value.clone()).collect(),
                            columns: columns
                                .iter()
                                .map(|c| {
                                    (
                                        c.name.value.clone(),
                                        Column {
                                            data: if c
                                                .options
                                                .iter()
                                                .any(|o| o.option == ColumnOption::NotNull)
                                            {
                                                match &c.data_type {
                                                    DataType::Real
                                                    | DataType::Double
                                                    | DataType::DoublePrecision
                                                    | DataType::Float(_) => {
                                                        ColumnData::Double(Vec::new())
                                                    }
                                                    DataType::Integer(_)
                                                    | DataType::SmallInt(_) => {
                                                        ColumnData::IntLike(Vec::new())
                                                    }
                                                    DataType::Varchar(_) | DataType::Date => {
                                                        ColumnData::StringLike(Vec::new())
                                                    }
                                                    a => panic!("type {a:?} not supported!"),
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
                                                    a => panic!("type {a:?} not supported!"),
                                                }
                                            },
                                        },
                                    )
                                })
                                .collect(),
                        },
                    );
                }
                sqlparser::ast::Statement::StartTransaction { .. }
                | sqlparser::ast::Statement::Commit { .. } => {}
                sqlparser::ast::Statement::Drop {
                    object_type: sqlparser::ast::ObjectType::Table,
                    if_exists: true,
                    names,
                    ..
                } => {
                    for n in names {
                        tables.remove(&n.0.last().unwrap().value);
                    }
                }
                s => println!("cargo:warning=Unsupported SQL statement: {s:?}"),
            }
        }
        Self { tables }
    }
}
