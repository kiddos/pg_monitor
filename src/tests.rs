#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::model::Statement;
    use clap::Parser;

    #[test]
    fn test_statement_formatting() {
        let stmt = Statement {
            userid: 10,
            dbid: 12345,
            queryid: 987654321,
            query: "SELECT * FROM users".to_string(),
            total_exec_time: 123.456,
            min_exec_time: 1.23,
            max_exec_time: 50.1,
            mean_exec_time: 12.34,
            stddev_exec_time: 5.67,
            rows: 100,
            calls: 10,
        };

        assert_eq!(stmt.formatted_total_time(), "123.46 ms");
        assert_eq!(stmt.formatted_mean_time(), "12.34 ms");
        assert_eq!(stmt.formatted_min_time(), "1.23 ms");
        assert_eq!(stmt.formatted_max_time(), "50.10 ms");
        assert_eq!(stmt.formatted_stddev_time(), "5.67 ms");
    }

    #[test]
    fn test_config_building() {
        // Individual args
        let args = vec![
            "pg_monitor",
            "-H", "localhost",
            "-U", "admin",
            "-w", "admin123",
            "-d", "test",
            "-p", "5432",
        ];
        let cfg = Config::parse_from(args);
        let pg_cfg = cfg.build_pg_config().expect("build_pg_config failed");

        // Verify pg_cfg debug output contains expected options
        let debug_str = format!("{:?}", pg_cfg);
        assert!(debug_str.contains("localhost"));
        assert!(debug_str.contains("admin"));
        assert!(debug_str.contains("test"));
    }
}
