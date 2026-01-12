use rustml::data::cleaning::{clean_dataset, CleaningConfig};
use rustml::data::dataset::Dataset;
use rustml::utils::cli::ArgParser;

fn main() {
    let parser = ArgParser::new();
    let input = parser
        .require("input")
        .expect("usage: --input <path> --output <path>");
    let output = parser
        .require("output")
        .expect("usage: --input <path> --output <path>");
    let dataset = Dataset::from_csv(&input).expect("failed to load input dataset");
    let cleaned = clean_dataset(&dataset, CleaningConfig::default()).expect("cleaning failed");
    cleaned.to_csv(&output).expect("failed to write cleaned dataset");
    println!("Cleaned dataset written to {output}");
}
