# RustML

RustML is a high-performance Rust library for Machine Learning, designed to handle and process large datasets efficiently. This project demonstrates Rust's capabilities for performance, safety, and concurrency in the context of ML tasks.

## Features

- Load and process datasets
- Train and evaluate machine learning models (Linear Regression, Logistic Regression)
- Normalize data and perform feature engineering
- Easily extendable and customizable

## Directory Structure

```plaintext
.
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── data/
│   │   ├── mod.rs
│   │   ├── dataset.rs
│   ├── ml/
│   │   ├── mod.rs
│   │   ├── linear_regression.rs
│   │   ├── logistic_regression.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── normalization.rs
│   │   ├── evaluation.rs
├── tests/
│   ├── data_tests.rs
│   ├── ml_tests.rs
├── Cargo.toml
├── README.md
```

## Getting Started
### Prerequisites
Rust 1.50.0 or higher

Cargo

### Installation
Clone the repository:


git clone https://github.com/your_username/rustml.git
cd rustml
Build the project:


cargo build
Run the main program:


cargo run

## Usage
### Data Generation
Generate synthetic data to simulate real-world network scenarios.


cargo run --bin data/synthetic_data_generator
Data Preprocessing
Clean, normalize, and engineer features for the generated data.

### Data Cleaning:

cargo run --bin preprocessing/data_cleaning
Normalization:


cargo run --bin preprocessing/normalization
### Feature Engineering:


cargo run --bin preprocessing/feature_engineering
### Model Training
Train various ML models for traffic prediction, anomaly detection, and network optimization.


cargo run --bin models/model_training
### Model Evaluation
Evaluate the trained models using different metrics and visualize the results.


cargo run --bin evaluation/model_evaluation
cargo run --bin evaluation/results_visualization
### Deployment
Deploy the models for real-time decision-making in SDN and NFV environments.

### SDN Controller Integration:


cargo run --bin deployment/sdn_controller_integration
### NFV Orchestrator Integration:


cargo run --bin deployment/nfv_orchestrator_integration
### Real-Time Decision Making:

cargo run --bin deployment/real_time_decision_making
