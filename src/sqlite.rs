use diesel::{sql_query, sqlite::SqliteConnection, RunQueryDsl};
use std::time::Duration;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        if self.enable_wal {
            sql_query("PRAGMA journal_mode = WAL")
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
            sql_query("PRAGMA synchronous = NORMAL")
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
        }
        if self.enable_foreign_keys {
            sql_query("PRAGMA foreign_keys = ON")
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
        }
        if let Some(d) = self.busy_timeout {
            sql_query(&format!("PRAGMA busy_timeout = {};", d.as_millis()))
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
        }
        Ok(())
    }
}
