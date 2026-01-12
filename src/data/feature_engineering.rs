use crate::data::dataset::{Dataset, DatasetError};

#[derive(Debug, Clone, Copy)]
pub struct FeatureEngineeringConfig {
    pub polynomial_degree: usize,
}

impl Default for FeatureEngineeringConfig {
    fn default() -> Self {
        FeatureEngineeringConfig { polynomial_degree: 2 }
    }
}

pub fn add_polynomial_features(
    dataset: &Dataset,
    config: FeatureEngineeringConfig,
) -> Result<Dataset, DatasetError> {
    if config.polynomial_degree < 2 {
        return Ok(dataset.clone());
    }
    let mut data = Vec::with_capacity(dataset.data.len());
    let mut feature_names = dataset.feature_names.clone();
    for degree in 2..=config.polynomial_degree {
        for name in dataset.feature_names.iter() {
            feature_names.push(format!("{name}^{}", degree));
        }
    }
    for row in &dataset.data {
        let mut expanded = row.clone();
        for degree in 2..=config.polynomial_degree {
            for value in row {
                expanded.push(value.powi(degree as i32));
            }
        }
        data.push(expanded);
    }
    Dataset::from_records(
        feature_names,
        dataset.target_name.clone(),
        data,
        dataset.target.clone(),
    )
}
