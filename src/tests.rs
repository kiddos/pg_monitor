#[cfg(test)]
mod tests {
    use crate::model::Statement;

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
}
