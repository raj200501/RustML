use rustml::data::synthetic::{generate_network_dataset, SyntheticConfig};
use rustml::deployment::sdn_decision;
use rustml::io::load_model;
use rustml::ml::Model;
use rustml::utils::cli::ArgParser;

fn main() {
    let parser = ArgParser::new();
    let model_path = parser
        .require("model")
        .expect("usage: --model <model.json> --output <decision.json>");
    let output = parser
        .require("output")
        .expect("usage: --model <model.json> --output <decision.json>");
    let rows = parser
        .optional("rows", "8")
        .parse::<usize>()
        .unwrap_or(8);
    let artifact = load_model(&model_path).expect("failed to load model");
    let dataset = generate_network_dataset(SyntheticConfig { rows, seed: 7 });
    let predictions = artifact.model.predict(&dataset);
    let avg_score = predictions.iter().sum::<f64>() / predictions.len().max(1) as f64;
    let decision = sdn_decision(avg_score);
    let content = serde_json::to_string_pretty(&decision).expect("serialize decision");
    std::fs::write(&output, content).expect("write decision");
    println!("Real-time decision written to {output}");
}
