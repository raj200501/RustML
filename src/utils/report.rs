use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub mse: f64,
    pub rmse: f64,
    pub mae: f64,
}
