use clap::Parser;
use std::str::FromStr;
use tokio_postgres::Config as PgConfig;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "PostgreSQL Query Monitor TUI tool", long_about = None)]
pub struct Config {
    /// PostgreSQL connection string (e.g. "host=localhost user=postgres password=secret dbname=postgres" or "postgres://user:pass@localhost:5432/dbname")
    #[arg(short = 'c', long, env = "DATABASE_URL")]
    pub connection_string: Option<String>,

    /// Database host (e.g. "localhost", "127.0.0.1", "/tmp")
    #[arg(short = 'H', long, env = "PGHOST")]
    pub host: Option<String>,

    /// Database port
    #[arg(short = 'p', long, env = "PGPORT")]
    pub port: Option<u16>,

    /// Database user
    #[arg(short = 'U', long, env = "PGUSER")]
    pub user: Option<String>,

    /// Database password
    #[arg(short = 'w', long, env = "PGPASSWORD")]
    pub password: Option<String>,

    /// Database name
    #[arg(short = 'd', long, env = "PGDATABASE")]
    pub dbname: Option<String>,

    /// Number of statements per page
    #[arg(short = 's', long, default_value_t = 20)]
    pub page_size: i64,

    /// Auto-refresh interval in seconds (0 to disable)
    #[arg(short = 'r', long, default_value_t = 5)]
    pub refresh_interval: u64,
}

impl Config {
    pub fn build_pg_config(&self) -> Result<PgConfig, tokio_postgres::Error> {
        let mut pg_config = if let Some(ref conn_str) = self.connection_string {
            PgConfig::from_str(conn_str)?
        } else {
            PgConfig::new()
        };

        if let Some(ref host) = self.host {
            pg_config.host(host);
        } else if self.connection_string.is_none() {
            pg_config.host("127.0.0.1");
        }

        if let Some(port) = self.port {
            pg_config.port(port);
        } else if self.connection_string.is_none() {
            pg_config.port(5432);
        }

        if let Some(ref user) = self.user {
            pg_config.user(user);
        } else if self.connection_string.is_none() {
            pg_config.user("postgres");
        }

        if let Some(ref password) = self.password {
            pg_config.password(password);
        }

        if let Some(ref dbname) = self.dbname {
            pg_config.dbname(dbname);
        } else if self.connection_string.is_none() {
            pg_config.dbname("postgres");
        }

        Ok(pg_config)
    }
}
