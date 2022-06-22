package app.kiddos.pg_monitor.controllers

import app.kiddos.pg_monitor.models.Statement
import app.kiddos.pg_monitor.models.StatementDAO
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/stat/v1")
class StatementController {
    @Autowired
    private lateinit var statementDAO: StatementDAO;

    @GetMapping("/")
    fun queryStatement(
        @RequestParam(value = "pageSize", defaultValue = "100") pageSize: Int,
        @RequestParam(value = "page", defaultValue = "1") page: Int) : List<Statement> {
        return statementDAO.queryStatement(pageSize, page)
    }
}