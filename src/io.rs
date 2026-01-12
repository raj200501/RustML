use crate::ml::linear_regression::LinearRegression;
use crate::pipeline::TrainingSummary;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelArtifact {
    pub model: LinearRegression,
    pub summary: TrainingSummary,
}

pub fn save_model(path: &str, artifact: &ModelArtifact) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_string_pretty(artifact)?;
    std::fs::write(path, content)?;
    Ok(())
}

pub fn load_model(path: &str) -> Result<ModelArtifact, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}
