# pg_monitor

A Rust terminal UI (TUI) tool to visualize PostgreSQL query statistics from `pg_stat_statements`.

## Setup

To enable `pg_stat_statements`, update `shared_preload_libraries` in `postgresql.conf`:

```conf
shared_preload_libraries = 'pg_stat_statements'
```

and execute:

```sql
CREATE EXTENSION pg_stat_statements;
```

## Build & Run

Ensure you have Rust installed.

### Build

```bash
cargo build --release
```

### Usage

```bash
# Connect using default options (host=localhost user=postgres dbname=postgres)
cargo run --release

# Or pass custom database connection string / arguments
cargo run --release -- --connection-string "host=localhost user=myuser password=mypass dbname=mydb" --page-size 20 --refresh-interval 5

# Or via DATABASE_URL environment variable
DATABASE_URL="host=localhost user=postgres dbname=postgres" cargo run --release
```

### Controls

- `▲ / ▼` or `j / k`: Select query in the table
- `◄ / ►` or `h / l` / `p / n`: Navigate pages
- `Enter` or `Space`: View detailed query view and full text
- `r`: Refresh query statistics immediately
- `q` or `Esc`: Close detail view / Quit application
