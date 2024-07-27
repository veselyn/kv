mod error;

use crate::database::Database;
use entity::key;
pub use error::*;
use extension::sqlite::SqliteExpr;
use sea_orm::{prelude::*, IntoIdentity, TransactionTrait};
use sea_query::*;
use std::collections::{HashMap, HashSet};

#[cfg_attr(test, derive(Default))]
#[derive(Debug)]
pub struct Repository {
    db: Database,
}

impl Repository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get<K>(&self, key: K) -> Result<Option<String>, GetError>
    where
        K: Into<String>,
    {
        let paths = self.get_paths(key, ["$"]).await?;

        let Some(mut paths) = paths else {
            return Ok(None);
        };

        let value = paths.remove("$").expect("paths does not contain root");

        Ok(Some(value))
    }

    pub async fn get_paths<K, P>(
        &self,
        key: K,
        paths: P,
    ) -> Result<Option<HashMap<String, String>>, GetError>
    where
        K: Into<String>,
        P: IntoIterator,
        P::Item: AsRef<str>,
    {
        let paths: HashSet<String> = paths
            .into_iter()
            .map(|path| path.as_ref().to_owned())
            .collect();

        let mut select_statement = Query::select();

        paths.iter().for_each(|path| {
            select_statement.expr_as(
                Expr::col(key::Column::Value).get_json_field(path),
                path.clone().into_identity(),
            );
        });

        select_statement
            .from(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key.into()));

        let result = self
            .db
            .query_one(self.db.get_database_backend().build(&select_statement))
            .await?;

        let Some(result) = result else {
            return Ok(None);
        };

        let paths = paths.iter().try_fold(HashMap::new(), |mut acc, path| {
            match result.try_get("", path) {
                Ok(value) => {
                    if let Some(value) = acc.insert(path.to_owned(), value) {
                        panic!("path {:?} has present value {value:?}", path);
                    }
                    Ok(acc)
                }
                Err(err) => match err {
                    DbErr::Type(msg)
                        if msg
                            == format!("A null value was encountered while decoding {path:?}") =>
                    {
                        Ok(acc)
                    }
                    _ => Err(err),
                },
            }
        })?;

        Ok(Some(paths))
    }

    pub async fn set<K, V>(&self, key: K, value: V) -> Result<(), SetError>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let insert_statement = Query::insert()
            .replace()
            .into_table(key::Entity)
            .columns([key::Column::Id, key::Column::Type, key::Column::Value])
            .values_panic([
                key.into().into(),
                "json".into(),
                Expr::cust_with_values("JSON(?)", [value.into()]),
            ])
            .to_owned();

        let result = self
            .db
            .execute(self.db.get_database_backend().build(&insert_statement))
            .await
            .map_err(|db_err| match db_err {
                DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(ref err))) => {
                    match (err.code().as_deref(), err.message()) {
                        (Some("1"), "malformed JSON") => SetError::MalformedJson(db_err),
                        _ => SetError::from(db_err),
                    }
                }
                err => SetError::from(err),
            })?;

        let affected = result.rows_affected();

        if affected != 1 {
            panic!(
                "{:?} rows were affected by set when expected 1 or 0",
                affected,
            )
        }

        Ok(())
    }

    pub async fn set_path<K, V, P>(&self, key: K, path: P, value: V) -> Result<Option<()>, SetError>
    where
        K: Into<String>,
        P: Into<String>,
        V: Into<String>,
    {
        let path = path.into();

        if path == "$" {
            self.set(key, value).await?;
            return Ok(Some(()));
        }

        let update_statement = Query::update()
            .table(key::Entity)
            .value(
                key::Column::Value,
                Expr::cust_with_exprs(
                    "JSON_SET(?, ?, JSON(?))",
                    [
                        Expr::col(key::Column::Value).into(),
                        Expr::val(path).into(),
                        Expr::val(value.into()).into(),
                    ],
                ),
            )
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key.into()))
            .to_owned();

        let result = self
            .db
            .execute(self.db.get_database_backend().build(&update_statement))
            .await
            .map_err(|db_err| match db_err {
                DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(ref err))) => {
                    match (err.code().as_deref(), err.message()) {
                        (Some("1"), "malformed JSON") => SetError::MalformedJson(db_err),
                        _ => SetError::from(db_err),
                    }
                }
                err => SetError::from(err),
            })?;

        let affected = result.rows_affected();

        Ok(match affected {
            1 => Some(()),
            0 => None,
            _ => panic!(
                "{:?} rows were affected by set when expected 1 or 0",
                affected,
            ),
        })
    }

    pub async fn del<K>(&self, key: K) -> Result<(), DelError>
    where
        K: Into<String>,
    {
        let key = key.into();

        let delete_statement = Query::delete()
            .from_table(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(&key))
            .to_owned();

        let result = self
            .db
            .execute(self.db.get_database_backend().build(&delete_statement))
            .await?;

        let affected = result.rows_affected();

        match affected {
            1 => Ok(()),
            0 => Err(DelError::KeyNotFound(key)),
            _ => panic!(
                "{:?} rows were affected by delete when expected 1 or 0",
                affected,
            ),
        }
    }

    pub async fn del_path<K, P>(&self, key: K, path: P) -> Result<Option<()>, DelError>
    where
        K: Into<String>,
        P: Into<String>,
    {
        let key = key.into();
        let path = path.into();

        if path == "$" {
            self.del(key).await?;
            return Ok(Some(()));
        }

        let txn = self.db.begin().await?;

        let select_key_exists_statement = Query::select()
            .expr_as(
                Expr::exists(
                    Query::select()
                        .column(key::Column::Id)
                        .from(key::Entity)
                        .and_where(key::Column::Type.eq("json"))
                        .and_where(key::Column::Id.eq(&key))
                        .take(),
                ),
                "key_exists".into_identity(),
            )
            .to_owned();

        let result = txn
            .query_one(
                txn.get_database_backend()
                    .build(&select_key_exists_statement),
            )
            .await?
            .expect("no result from key exists query");

        if !result.try_get("", "key_exists")? {
            return Err(DelError::KeyNotFound(key));
        }

        let select_path_exists_statement = Query::select()
            .expr_as(
                Expr::exists(
                    Query::select()
                        .column(key::Column::Id)
                        .from(key::Entity)
                        .and_where(key::Column::Type.eq("json"))
                        .and_where(key::Column::Id.eq(&key))
                        .and_where(
                            Expr::expr(Expr::cust_with_exprs(
                                "JSON_TYPE(?, ?)",
                                [
                                    Expr::col(key::Column::Value).into(),
                                    Expr::val(&path).into(),
                                ],
                            ))
                            .is_not_null(),
                        )
                        .take(),
                ),
                "path_exists".into_identity(),
            )
            .to_owned();

        let result = txn
            .query_one(
                txn.get_database_backend()
                    .build(&select_path_exists_statement),
            )
            .await?
            .expect("no result from path exists query");

        if !result.try_get("", "path_exists")? {
            return Ok(None);
        }

        let update_statement = Query::update()
            .table(key::Entity)
            .value(
                key::Column::Value,
                Expr::cust_with_exprs(
                    "JSON_REMOVE(?, ?)",
                    [
                        Expr::col(key::Column::Value).into(),
                        Expr::val(&path).into(),
                    ],
                ),
            )
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(&key))
            .to_owned();

        let result = txn
            .execute(txn.get_database_backend().build(&update_statement))
            .await?;

        let affected = result.rows_affected();

        let result = match affected {
            1 => Some(()),
            0 => None,
            _ => panic!(
                "{:?} rows were affected by delete when expected 1 or 0",
                affected,
            ),
        };

        txn.commit().await?;

        Ok(result)
    }
}
