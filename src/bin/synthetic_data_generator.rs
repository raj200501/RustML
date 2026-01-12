use rustml::data::synthetic::{generate_network_dataset, SyntheticConfig};
use rustml::utils::cli::ArgParser;

fn main() {
    let parser = ArgParser::new();
    let output = parser.optional("output", "output/synthetic.csv");
    let rows = parser
        .optional("rows", "256")
        .parse::<usize>()
        .unwrap_or(256);
    let seed = parser
        .optional("seed", "42")
        .parse::<u64>()
        .unwrap_or(42);

    let dataset = generate_network_dataset(SyntheticConfig { rows, seed });
    if let Some(parent) = std::path::Path::new(&output).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("failed to create output directory");
        }
    }
    dataset.to_csv(&output).expect("failed to write dataset");
    println!("Synthetic dataset written to {output}");
}
