use crate::database::Database;
use entity::key;
use sea_orm::prelude::*;
use sea_query::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum SetError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum DelError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[cfg_attr(test, derive(Default))]
#[derive(Debug)]
pub struct Repository {
    db: Database,
}

impl Repository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get<S>(&self, key: S) -> Result<Option<String>, GetError>
    where
        S: Into<String>,
    {
        let select_statement = Query::select()
            .expr_as(
                Expr::cust_with_expr("JSON(?)", Expr::col(key::Column::Value)),
                key::Column::Value,
            )
            .from(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key.into()))
            .to_owned();

        let db = self.db.get();
        let result = db
            .query_one(db.get_database_backend().build(&select_statement))
            .await?;

        let value = result
            .map(|result| result.try_get("", "value"))
            .transpose()?;

        Ok(value)
    }

    pub async fn set<S>(&self, key: S, value: S) -> Result<(), SetError>
    where
        S: Into<String>,
    {
        let insert_statement = Query::insert()
            .replace()
            .into_table(key::Entity)
            .columns([key::Column::Id, key::Column::Type, key::Column::Value])
            .values_panic([
                key.into().into(),
                "json".into(),
                Expr::cust_with_expr("JSON(?)", value.into()),
            ])
            .to_owned();

        let db = self.db.get();
        db.execute(db.get_database_backend().build(&insert_statement))
            .await?;

        Ok(())
    }

    pub async fn del<S>(&self, key: S) -> Result<(), DelError>
    where
        S: Into<String>,
    {
        let delete_statement = Query::delete()
            .from_table(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key.into()))
            .to_owned();

        let db = self.db.get();
        db.execute(db.get_database_backend().build(&delete_statement))
            .await?;

        Ok(())
    }
}
