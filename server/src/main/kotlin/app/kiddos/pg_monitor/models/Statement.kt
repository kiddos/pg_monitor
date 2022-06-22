package app.kiddos.pg_monitor.models

class Statement constructor(
    val userId: Int,
    val dbId: Int,
    val queryId: Long,
    val query: String,
    val totalExecTime: Double,
    val minExecTime: Double,
    val maxExecTime: Double,
    val meanExecTime: Double,
    val stddevExecTime: Double,
    val row: Long,
    val calls: Long) {
}