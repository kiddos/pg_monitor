package app.kiddos.pg_monitor.models

import org.springframework.beans.factory.annotation.Autowired
import org.springframework.jdbc.core.JdbcTemplate
import org.springframework.stereotype.Component

@Component
class StatementDAO {
    @Autowired
    private lateinit var jdbcTemplate: JdbcTemplate

    fun queryStatement(pageSize: Int, page: Int) : List<Statement> {
        return jdbcTemplate.query("""
            SELECT userid, dbid, queryid, query,
                total_exec_time, min_exec_time, max_exec_time, mean_exec_time, stddev_exec_time,
                rows, calls
                FROM pg_stat_statements
                ORDER BY total_exec_time DESC
                LIMIT $pageSize OFFSET $page
        """.trimIndent()) { rs, _ ->
            Statement(
                rs.getInt("userid"),
                rs.getInt("dbid"),
                rs.getLong("queryid"),
                rs.getString("query"),
                rs.getDouble("total_exec_time"),
                rs.getDouble("min_exec_time"),
                rs.getDouble("max_exec_time"),
                rs.getDouble("mean_exec_time"),
                rs.getDouble("stddev_exec_time"),
                rs.getLong("rows"),
                rs.getLong("calls")
            )
        };
    }
}