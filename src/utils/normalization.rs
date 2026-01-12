use crate::data::dataset::{Dataset, DatasetError};

#[derive(Debug, Clone)]
pub struct NormalizationStats {
    pub mean: Vec<f64>,
    pub std_dev: Vec<f64>,
    pub min: Vec<f64>,
    pub max: Vec<f64>,
}

impl NormalizationStats {
    pub fn z_score(dataset: &Dataset) -> Self {
        let cols = dataset.num_features();
        let mut mean = vec![0.0; cols];
        let mut std_dev = vec![0.0; cols];
        let mut min = vec![f64::MAX; cols];
        let mut max = vec![f64::MIN; cols];
        for row in &dataset.data {
            for (idx, value) in row.iter().enumerate() {
                mean[idx] += value;
                min[idx] = min[idx].min(*value);
                max[idx] = max[idx].max(*value);
            }
        }
        for value in &mut mean {
            *value /= dataset.data.len() as f64;
        }
        for row in &dataset.data {
            for (idx, value) in row.iter().enumerate() {
                std_dev[idx] += (*value - mean[idx]).powi(2);
            }
        }
        for value in &mut std_dev {
            *value = (*value / dataset.data.len() as f64).sqrt().max(1e-12);
        }
        NormalizationStats {
            mean,
            std_dev,
            min,
            max,
        }
    }
}

pub fn normalize_z_score(dataset: &Dataset) -> Result<(Dataset, NormalizationStats), DatasetError> {
    let stats = NormalizationStats::z_score(dataset);
    let mut data = dataset.data.clone();
    for row in &mut data {
        for (idx, value) in row.iter_mut().enumerate() {
            *value = (*value - stats.mean[idx]) / stats.std_dev[idx];
        }
    }
    Ok((
        Dataset::from_records(
            dataset.feature_names.clone(),
            dataset.target_name.clone(),
            data,
            dataset.target.clone(),
        )?,
        stats,
    ))
}

pub fn normalize_min_max(dataset: &Dataset) -> Result<Dataset, DatasetError> {
    let stats = NormalizationStats::z_score(dataset);
    let mut data = dataset.data.clone();
    for row in &mut data {
        for (idx, value) in row.iter_mut().enumerate() {
            let range = (stats.max[idx] - stats.min[idx]).max(1e-12);
            *value = (*value - stats.min[idx]) / range;
        }
    }
    Dataset::from_records(
        dataset.feature_names.clone(),
        dataset.target_name.clone(),
        data,
        dataset.target.clone(),
    )
}
