use rustml::data::cleaning::{clean_dataset, CleaningConfig};
use rustml::data::dataset::Dataset;
use rustml::data::feature_engineering::{add_polynomial_features, FeatureEngineeringConfig};
use rustml::data::fixtures::fixture_dataset;
use rustml::data::synthetic::{generate_network_dataset, SyntheticConfig};
use rustml::utils::normalization::{normalize_min_max, normalize_z_score};

#[test]
fn test_from_csv() {
    let dataset = Dataset::from_csv("data/iris.csv").unwrap();
    assert_eq!(dataset.data.len(), 150);
    assert_eq!(dataset.target.len(), 150);
    assert_eq!(dataset.num_features(), 4);
}

#[test]
fn test_synthetic_dataset_generation() {
    let dataset = generate_network_dataset(SyntheticConfig { rows: 10, seed: 1 });
    assert_eq!(dataset.num_rows(), 10);
    assert_eq!(dataset.num_features(), 6);
}

#[test]
fn test_cleaning_pipeline() {
    let dataset = generate_network_dataset(SyntheticConfig { rows: 32, seed: 2 });
    let cleaned = clean_dataset(&dataset, CleaningConfig::default()).unwrap();
    assert_eq!(cleaned.num_rows(), 32);
}

#[test]
fn test_normalization_z_score() {
    let dataset = generate_network_dataset(SyntheticConfig { rows: 20, seed: 3 });
    let (normalized, stats) = normalize_z_score(&dataset).unwrap();
    assert_eq!(normalized.num_features(), dataset.num_features());
    assert_eq!(stats.mean.len(), dataset.num_features());
}

#[test]
fn test_normalization_min_max() {
    let dataset = generate_network_dataset(SyntheticConfig { rows: 20, seed: 4 });
    let normalized = normalize_min_max(&dataset).unwrap();
    assert_eq!(normalized.num_features(), dataset.num_features());
}

#[test]
fn test_feature_engineering() {
    let dataset = generate_network_dataset(SyntheticConfig { rows: 8, seed: 5 });
    let engineered =
        add_polynomial_features(&dataset, FeatureEngineeringConfig { polynomial_degree: 3 })
            .unwrap();
    assert!(engineered.num_features() > dataset.num_features());
}

#[test]
fn test_fixture_dataset() {
    let dataset = fixture_dataset();
    assert_eq!(dataset.num_rows(), 5000);
    assert_eq!(dataset.num_features(), 6);
}
