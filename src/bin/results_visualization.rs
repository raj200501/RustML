use rustml::utils::cli::ArgParser;
use rustml::utils::report::EvaluationReport;

fn main() {
    let parser = ArgParser::new();
    let input = parser
        .require("input")
        .expect("usage: --input <report.json>");
    let content = std::fs::read_to_string(&input).expect("failed to read report");
    let report: EvaluationReport = serde_json::from_str(&content).expect("invalid report");
    println!(
        "Evaluation Summary\\nMSE: {:.4}\\nRMSE: {:.4}\\nMAE: {:.4}",
        report.mse, report.rmse, report.mae
    );
}
