use rustml::deployment::sdn_decision;
use rustml::io::load_model;
use rustml::utils::cli::ArgParser;

fn main() {
    let parser = ArgParser::new();
    let model_path = parser
        .require("model")
        .expect("usage: --model <model.json> --output <decision.json>");
    let output = parser
        .require("output")
        .expect("usage: --model <model.json> --output <decision.json>");
    let artifact = load_model(&model_path).expect("failed to load model");
    let score = artifact.summary.mse + artifact.summary.rmse + artifact.summary.mae;
    let decision = sdn_decision(score);
    let content = serde_json::to_string_pretty(&decision).expect("serialize decision");
    std::fs::write(&output, content).expect("write decision");
    println!("SDN decision written to {output}");
}
