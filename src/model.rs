use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Statement {
    pub userid: i64,
    pub dbid: i64,
    pub queryid: i64,
    pub query: String,
    pub total_exec_time: f64,
    pub min_exec_time: f64,
    pub max_exec_time: f64,
    pub mean_exec_time: f64,
    pub stddev_exec_time: f64,
    pub rows: i64,
    pub calls: i64,
}

impl Statement {
    pub fn formatted_total_time(&self) -> String {
        format!("{:.2} ms", self.total_exec_time)
    }

    pub fn formatted_mean_time(&self) -> String {
        format!("{:.2} ms", self.mean_exec_time)
    }

    pub fn formatted_min_time(&self) -> String {
        format!("{:.2} ms", self.min_exec_time)
    }

    pub fn formatted_max_time(&self) -> String {
        format!("{:.2} ms", self.max_exec_time)
    }

    pub fn formatted_stddev_time(&self) -> String {
        format!("{:.2} ms", self.stddev_exec_time)
    }
}
