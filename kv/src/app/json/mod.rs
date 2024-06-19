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

        self.db
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "INSERT OR REPLACE INTO key (id, type, value) VALUES ($1, 'json', json($2))",
                [key.into(), value.into()],
            ))
            .await?;

        Ok(())
    }

    pub async fn json_del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        let key = key.into();

        self.db
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "DELETE FROM key WHERE id = $1 AND type = 'json'",
                [key.into()],
            ))
            .await?;

        Ok(())
    }
}
