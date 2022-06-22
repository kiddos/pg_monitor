package app.kiddos.pg_monitor

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class PgMonitorApplication

fun main(args: Array<String>) {
	runApplication<PgMonitorApplication>(*args)
}
