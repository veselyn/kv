use entity::key;
use sea_orm::prelude::*;
use sea_query::*;

#[derive(Debug)]
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get<S>(&self, key: S) -> anyhow::Result<Option<String>>
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

        let result = self
            .db
            .query_one(self.db.get_database_backend().build(&select_statement))
            .await?;

        let value = result
            .map(|result| result.try_get("", "value"))
            .transpose()?;

        Ok(value)
    }

    pub async fn set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        let insert_statement = Query::insert()
            .replace()
            .into_table(key::Entity)
            .columns([key::Column::Id, key::Column::Type, key::Column::Value])
            .values([
                key.into().into(),
                "json".into(),
                Expr::cust_with_expr("JSON(?)", value.into()),
            ])?
            .to_owned();

        self.db
            .execute(self.db.get_database_backend().build(&insert_statement))
            .await?;

        Ok(())
    }

    pub async fn del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        let delete_statement = Query::delete()
            .from_table(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key.into()))
            .to_owned();

        self.db
            .execute(self.db.get_database_backend().build(&delete_statement))
            .await?;

        Ok(())
    }
}
