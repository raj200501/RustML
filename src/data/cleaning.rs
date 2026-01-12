use crate::data::dataset::{Dataset, DatasetError};

#[derive(Debug, Clone, Copy)]
pub struct CleaningConfig {
    pub missing_value_threshold: f64,
    pub clip_lower_percentile: f64,
    pub clip_upper_percentile: f64,
}

impl Default for CleaningConfig {
    fn default() -> Self {
        CleaningConfig {
            missing_value_threshold: 0.0,
            clip_lower_percentile: 0.01,
            clip_upper_percentile: 0.99,
        }
    }
}

pub fn remove_rows_with_missing(dataset: &Dataset) -> Result<Dataset, DatasetError> {
    let mut data = Vec::new();
    let mut target = Vec::new();
    for (row, target_value) in dataset.iter_rows() {
        if row.iter().all(|value| value.is_finite()) && target_value.is_finite() {
            data.push(row.to_vec());
            target.push(target_value);
        }
    }
    if data.is_empty() {
        return Err(DatasetError::EmptyDataset);
    }
    Dataset::from_records(
        dataset.feature_names.clone(),
        dataset.target_name.clone(),
        data,
        target,
    )
}

pub fn clip_outliers(dataset: &Dataset, config: CleaningConfig) -> Result<Dataset, DatasetError> {
    let mut clipped = dataset.data.clone();
    let cols = dataset.num_features();
    for col_idx in 0..cols {
        let mut values: Vec<f64> = dataset.data.iter().map(|row| row[col_idx]).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let lower_idx = ((values.len() as f64) * config.clip_lower_percentile).floor() as usize;
        let upper_idx = ((values.len() as f64) * config.clip_upper_percentile).ceil() as usize;
        let lower = values[lower_idx.min(values.len() - 1)];
        let upper = values[upper_idx.min(values.len() - 1)];
        for row in &mut clipped {
            if row[col_idx] < lower {
                row[col_idx] = lower;
            }
            if row[col_idx] > upper {
                row[col_idx] = upper;
            }
        }
    }
    Dataset::from_records(
        dataset.feature_names.clone(),
        dataset.target_name.clone(),
        clipped,
        dataset.target.clone(),
    )
}

pub fn clean_dataset(dataset: &Dataset, config: CleaningConfig) -> Result<Dataset, DatasetError> {
    let removed = remove_rows_with_missing(dataset)?;
    clip_outliers(&removed, config)
}
