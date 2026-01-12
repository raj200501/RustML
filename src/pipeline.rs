use crate::data::cleaning::{clean_dataset, CleaningConfig};
use crate::data::dataset::Dataset;
use crate::ml::linear_regression::LinearRegression;
use crate::ml::Model;
use crate::utils::evaluation::{mean_squared_error, mean_absolute_error, root_mean_squared_error};
use crate::utils::normalization::{normalize_z_score, NormalizationStats};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSummary {
    pub mse: f64,
    pub rmse: f64,
    pub mae: f64,
    pub epochs: usize,
    pub loss: f64,
}

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub model: LinearRegression,
    pub normalized: Dataset,
    pub stats: NormalizationStats,
    pub summary: TrainingSummary,
}

pub fn train_linear_pipeline(dataset: &Dataset) -> Result<PipelineResult, crate::data::dataset::DatasetError> {
    let cleaned = clean_dataset(dataset, CleaningConfig::default())?;
    let (normalized, stats) = normalize_z_score(&cleaned)?;
    let mut model = LinearRegression::new(normalized.num_features());
    let report = model.train(&normalized);
    let predictions = model.predict(&normalized);
    let mse = mean_squared_error(&predictions, &normalized.target);
    let rmse = root_mean_squared_error(&predictions, &normalized.target);
    let mae = mean_absolute_error(&predictions, &normalized.target);
    let summary = TrainingSummary {
        mse,
        rmse,
        mae,
        epochs: report.epochs,
        loss: report.final_loss,
    };
    Ok(PipelineResult {
        model,
        normalized,
        stats,
        summary,
    })
}
