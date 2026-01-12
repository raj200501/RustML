use rustml::data::dataset::Dataset;
use rustml::io::{save_model, ModelArtifact};
use rustml::pipeline::train_linear_pipeline;
use rustml::utils::cli::ArgParser;

fn main() {
    let parser = ArgParser::new();
    let input = parser
        .require("input")
        .expect("usage: --input <path> --output <model.json>");
    let output = parser
        .require("output")
        .expect("usage: --input <path> --output <model.json>");

    let dataset = Dataset::from_csv(&input).expect("failed to load dataset");
    let result = train_linear_pipeline(&dataset).expect("training pipeline failed");
    let artifact = ModelArtifact {
        model: result.model,
        summary: result.summary,
    };
    save_model(&output, &artifact).expect("failed to write model artifact");
    println!("Model artifact written to {output}");
}
