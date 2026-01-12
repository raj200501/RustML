use rustml::data::dataset::Dataset;
use rustml::data::synthetic::{generate_network_dataset, SyntheticConfig};
use rustml::ml::linear_regression::LinearRegression;
use rustml::ml::logistic_regression::LogisticRegression;
use rustml::ml::Model;
use rustml::pipeline::train_linear_pipeline;

#[test]
fn test_linear_regression_training() {
    let dataset = generate_network_dataset(SyntheticConfig { rows: 64, seed: 10 });
    let mut model = LinearRegression::new(dataset.num_features());
    let report = model.train(&dataset);
    assert!(report.final_loss.is_finite());
    let predictions = model.predict(&dataset);
    assert_eq!(predictions.len(), dataset.num_rows());
}

#[test]
fn test_logistic_regression_training() {
    let mut dataset = generate_network_dataset(SyntheticConfig { rows: 40, seed: 11 });
    dataset.target = dataset
        .target
        .iter()
        .map(|value| if *value > 20.0 { 1.0 } else { 0.0 })
        .collect();
    let mut model = LogisticRegression::new(dataset.num_features());
    let report = model.train(&dataset);
    assert!(report.final_loss.is_finite());
    let predictions = model.predict(&dataset);
    assert_eq!(predictions.len(), dataset.num_rows());
}

#[test]
fn test_pipeline_training() {
    let dataset = Dataset::from_csv("data/iris.csv").unwrap();
    let result = train_linear_pipeline(&dataset).unwrap();
    assert!(result.summary.mse.is_finite());
}
