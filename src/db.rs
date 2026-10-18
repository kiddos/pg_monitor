use anyhow::{Context, Result};
use tokio_postgres::{Client, Config as PgConfig, NoTls};
use crate::model::Statement;

pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub async fn connect(pg_config: &PgConfig) -> Result<Self> {
        let (client, connection) = pg_config
            .connect(NoTls)
            .await
            .with_context(|| "Failed to connect to PostgreSQL database")?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection background error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn fetch_statements(&self, page_size: i64, page: i64) -> Result<Vec<Statement>> {
        let sql = "SELECT userid, dbid, queryid, query, \
                    total_exec_time, min_exec_time, max_exec_time, mean_exec_time, stddev_exec_time, \
                    rows, calls \
                    FROM pg_stat_statements \
                    ORDER BY total_exec_time DESC \
                    LIMIT $1 OFFSET $2";

        let offset = page * page_size;
        let rows = self.client.query(sql, &[&page_size, &offset]).await
            .with_context(|| "Failed executing query on pg_stat_statements. Is pg_stat_statements extension created?")?;

        let mut statements = Vec::new();
        for row in rows {
            let userid: i64 = row.try_get::<_, i64>(0)
                .or_else(|_| row.try_get::<_, i32>(0).map(|v| v as i64))
                .or_else(|_| row.try_get::<_, u32>(0).map(|v| v as i64))
                .unwrap_or(0);

            let dbid: i64 = row.try_get::<_, i64>(1)
                .or_else(|_| row.try_get::<_, i32>(1).map(|v| v as i64))
                .or_else(|_| row.try_get::<_, u32>(1).map(|v| v as i64))
                .unwrap_or(0);

            let queryid: i64 = row.try_get::<_, i64>(2).unwrap_or(0);
            let query: String = row.try_get::<_, String>(3).unwrap_or_default();
            let total_exec_time: f64 = row.try_get::<_, f64>(4).unwrap_or(0.0);
            let min_exec_time: f64 = row.try_get::<_, f64>(5).unwrap_or(0.0);
            let max_exec_time: f64 = row.try_get::<_, f64>(6).unwrap_or(0.0);
            let mean_exec_time: f64 = row.try_get::<_, f64>(7).unwrap_or(0.0);
            let stddev_exec_time: f64 = row.try_get::<_, f64>(8).unwrap_or(0.0);
            let rows_cnt: i64 = row.try_get::<_, i64>(9).unwrap_or(0);
            let calls: i64 = row.try_get::<_, i64>(10).unwrap_or(0);

            statements.push(Statement {
                userid,
                dbid,
                queryid,
                query,
                total_exec_time,
                min_exec_time,
                max_exec_time,
                mean_exec_time,
                stddev_exec_time,
                rows: rows_cnt,
                calls,
            });
        }

        Ok(statements)
    }
}
