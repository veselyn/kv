mod jq;
#[cfg(test)]
mod tests;

use ::entity::key;
use sea_orm::*;
use sea_query::*;

use crate::app::App;

impl App {
    pub async fn json_get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        let key = key.into();

        let select_statement = Query::select()
            .expr_as(
                Expr::cust_with_expr("JSON(?)", Expr::col(key::Column::Value)),
                key::Column::Value,
            )
            .from(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key))
            .to_owned();

        let result = self
            .db
            .query_one(self.db.get_database_backend().build(&select_statement))
            .await?;

        let Some(result) = result else {
            anyhow::bail!("key does not exist")
        };

        let value: String = result.try_get("", "value")?;

        let formatted = jq::format(value)?;

        Ok(formatted)
    }

    pub async fn json_set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        let insert_statement = Query::insert()
            .replace()
            .into_table(key::Entity)
            .columns([key::Column::Id, key::Column::Type, key::Column::Value])
            .values([
                key.into(),
                "json".into(),
                Expr::cust_with_expr("JSON(?)", value),
            ])?
            .to_owned();

        self.db
            .execute(self.db.get_database_backend().build(&insert_statement))
            .await?;

        Ok(())
    }

    pub async fn json_del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        let key = key.into();

        let delete_statement = Query::delete()
            .from_table(key::Entity)
            .and_where(key::Column::Type.eq("json"))
            .and_where(key::Column::Id.eq(key))
            .to_owned();

        self.db
            .execute(self.db.get_database_backend().build(&delete_statement))
            .await?;

        Ok(())
    }
}
