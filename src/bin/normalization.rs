use rustml::data::dataset::Dataset;
use rustml::utils::cli::ArgParser;
use rustml::utils::normalization::normalize_z_score;

fn main() {
    let parser = ArgParser::new();
    let input = parser
        .require("input")
        .expect("usage: --input <path> --output <path>");
    let output = parser
        .require("output")
        .expect("usage: --input <path> --output <path>");
    let dataset = Dataset::from_csv(&input).expect("failed to load input dataset");
    let (normalized, _) = normalize_z_score(&dataset).expect("normalization failed");
    normalized
        .to_csv(&output)
        .expect("failed to write normalized dataset");
    println!("Normalized dataset written to {output}");
}
