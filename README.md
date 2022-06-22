# pg_monitor
monitor postgres queries

## setup

to enable `pg_stat_statements`, update `shared_preload_libraries` in `postgresql.conf`

```conf
shared_preload_libraries = 'pg_stat_statements'
```

and execute

```sql
CREATE EXTENSION pg_stat_statements
```
