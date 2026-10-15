use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "PostgreSQL Query Monitor TUI tool", long_about = None)]
pub struct Config {
    /// PostgreSQL connection string (e.g. "host=localhost user=postgres password=secret dbname=postgres")
    #[arg(short, long, env = "DATABASE_URL", default_value = "host=localhost user=postgres dbname=postgres")]
    pub connection_string: String,

    /// Number of statements per page
    #[arg(short, long, default_value_t = 20)]
    pub page_size: i64,

    /// Auto-refresh interval in seconds (0 to disable)
    #[arg(short, long, default_value_t = 5)]
    pub refresh_interval: u64,
}
