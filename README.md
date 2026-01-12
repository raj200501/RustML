# RustML

RustML is a Rust-first machine learning toolkit focused on reproducible data pipelines, model training, and integration workflows. It ships with a runnable demo pipeline, command-line tools for each stage, and a deterministic verification script suitable for CI.

## Features

- Load and process CSV datasets.
- Clean data, normalize features, and engineer polynomial features.
- Train linear and logistic regression models with deterministic behavior.
- Evaluate models with common regression metrics.
- Simulate SDN/NFV deployment decisions.

## Project Layout

```plaintext
.
├── data/
│   └── iris.csv
├── scripts/
│   ├── run.sh
│   ├── smoke_test.sh
│   └── verify.sh
├── src/
│   ├── bin/
│   ├── data/
│   ├── deployment/
│   ├── math/
│   ├── ml/
│   ├── utils/
│   ├── config.rs
│   ├── io.rs
│   ├── lib.rs
│   └── main.rs
└── tests/
```

## Prerequisites

- Rust (stable toolchain)
- Cargo (ships with Rust)

## Installation

```bash
git clone <your fork or clone url>
cd RustML
cargo build
```

## Usage

### Run the demo pipeline

The default demo runs a full training pipeline against `data/iris.csv`.

```bash
./scripts/run.sh
```

### Generate synthetic data

```bash
cargo run --bin synthetic_data_generator -- --output output/synthetic.csv --rows 256 --seed 42
```

### Data cleaning

```bash
cargo run --bin data_cleaning -- --input output/synthetic.csv --output output/cleaned.csv
```

### Normalization

```bash
cargo run --bin normalization -- --input output/cleaned.csv --output output/normalized.csv
```

### Feature engineering

```bash
cargo run --bin feature_engineering -- --input output/normalized.csv --output output/features.csv --degree 2
```

### Model training

```bash
cargo run --bin model_training -- --input output/features.csv --output output/model.json
```

### Model evaluation

```bash
cargo run --bin model_evaluation -- --input output/features.csv --model output/model.json --output output/report.json
```

### Results visualization

```bash
cargo run --bin results_visualization -- --input output/report.json
```

### Deployment simulation

```bash
cargo run --bin sdn_controller_integration -- --model output/model.json --output output/sdn_decision.json
cargo run --bin nfv_orchestrator_integration -- --model output/model.json --output output/nfv_decision.json
cargo run --bin real_time_decision_making -- --model output/model.json --output output/realtime_decision.json --rows 8
```

## Verified Quickstart

Commands executed successfully:

```bash
./scripts/run.sh
```

## Verified Verification

The canonical verification entrypoint is:

```bash
./scripts/verify.sh
```

It runs the unit tests and a smoke test that exercises the end-to-end pipeline.

## Troubleshooting

- If `cargo run` fails to find `data/iris.csv`, ensure you are running from the repository root.
- If outputs are missing, check that the `output/` directory is writable.
