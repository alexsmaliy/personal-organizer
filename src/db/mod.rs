#![cfg(feature = "ssr")]

use snafu::prelude::*;
use sqlx::{Acquire, Execute, Executor, Pool, Sqlite, sqlite::SqliteQueryResult};
use crate::errors::{DbCtx, Error, ComboCtx};

pub(super) const SQLITE_DB_FILENAME: &str = "omark.db";

pub(super) const INITIALIZE_SCHEMA: &str = include_str!("schema.sql");

pub(super) const DB_INIT_PRAGMAS: &str = "
    PRAGMA journal_mode = WAL;
";

pub(super) const CONNECTION_INIT_PRAGMAS: &str = "
    PRAGMA foreign_keys = ON;
";

pub(crate)
async fn run_in_txn<'res, 'query, S>
    (db_pool: &Pool<Sqlite>, raw_stmt: S, span: &'static str) -> Result<SqliteQueryResult, Error>
where 'query: 'res,
      S: AsRef<str> + Execute<'res, Sqlite> + 'query {

        let ctx = DbCtx { span };

        let mut conn = db_pool.acquire().await.context(ctx)?;
        let mut txn = conn.begin().await.context(ctx)?;

        match txn.execute(raw_stmt).await.context(ctx) {
            Ok(res) => txn.commit().await.map(|_| res).context(ctx),
            Err(e1) => match txn.rollback().await {
                Ok(_) => Err(e1),
                e2 => {
                    let e2 = e2.context(ctx)
                               .context(ComboCtx { span, also: Box::new(e1) })
                               .unwrap_err();
                    Err(e2)
                }
            }
        }
}

pub(crate)
async fn run<'result, 'query, S>
    (db_pool: &Pool<Sqlite>, raw_stmt: S, span: &'static str) -> Result<SqliteQueryResult, Error>
where 'query: 'result,
      S: AsRef<str> + Execute<'result, Sqlite> + 'query {
        db_pool.execute(raw_stmt).await.context(DbCtx { span })
}
