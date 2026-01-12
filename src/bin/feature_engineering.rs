use rustml::data::dataset::Dataset;
use rustml::data::feature_engineering::{add_polynomial_features, FeatureEngineeringConfig};
use rustml::utils::cli::ArgParser;

fn main() {
    let parser = ArgParser::new();
    let input = parser
        .require("input")
        .expect("usage: --input <path> --output <path> [--degree <n>]");
    let output = parser
        .require("output")
        .expect("usage: --input <path> --output <path> [--degree <n>]");
    let degree = parser
        .optional("degree", "2")
        .parse::<usize>()
        .unwrap_or(2);
    let dataset = Dataset::from_csv(&input).expect("failed to load input dataset");
    let engineered = add_polynomial_features(&dataset, FeatureEngineeringConfig { polynomial_degree: degree })
        .expect("feature engineering failed");
    engineered
        .to_csv(&output)
        .expect("failed to write feature engineered dataset");
    println!("Feature engineered dataset written to {output}");
}
