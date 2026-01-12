use rustml::data::dataset::Dataset;
use rustml::io::load_model;
use rustml::ml::Model;
use rustml::utils::cli::ArgParser;
use rustml::utils::evaluation::{mean_absolute_error, mean_squared_error, root_mean_squared_error};
use rustml::utils::report::EvaluationReport;

fn main() {
    let parser = ArgParser::new();
    let input = parser
        .require("input")
        .expect("usage: --input <path> --model <model.json> --output <report.json>");
    let model_path = parser
        .require("model")
        .expect("usage: --input <path> --model <model.json> --output <report.json>");
    let output = parser
        .require("output")
        .expect("usage: --input <path> --model <model.json> --output <report.json>");

    let dataset = Dataset::from_csv(&input).expect("failed to load dataset");
    let artifact = load_model(&model_path).expect("failed to load model artifact");
    let predictions = artifact.model.predict(&dataset);
    let report = EvaluationReport {
        mse: mean_squared_error(&predictions, &dataset.target),
        rmse: root_mean_squared_error(&predictions, &dataset.target),
        mae: mean_absolute_error(&predictions, &dataset.target),
    };
    let content = serde_json::to_string_pretty(&report).expect("failed to serialize report");
    std::fs::write(&output, content).expect("failed to write report");
    println!("Evaluation report written to {output}");
}
