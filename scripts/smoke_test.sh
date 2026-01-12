#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

WORK_DIR="${ROOT_DIR}/output/smoke"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"

echo "Generating synthetic dataset..."
cargo run --bin synthetic_data_generator -- --output "$WORK_DIR/synthetic.csv" --rows 64 --seed 7

echo "Cleaning dataset..."
cargo run --bin data_cleaning -- --input "$WORK_DIR/synthetic.csv" --output "$WORK_DIR/cleaned.csv"

echo "Normalizing dataset..."
cargo run --bin normalization -- --input "$WORK_DIR/cleaned.csv" --output "$WORK_DIR/normalized.csv"

echo "Feature engineering dataset..."
cargo run --bin feature_engineering -- --input "$WORK_DIR/normalized.csv" --output "$WORK_DIR/features.csv" --degree 2

echo "Training model..."
cargo run --bin model_training -- --input "$WORK_DIR/features.csv" --output "$WORK_DIR/model.json"

echo "Evaluating model..."
cargo run --bin model_evaluation -- --input "$WORK_DIR/features.csv" --model "$WORK_DIR/model.json" --output "$WORK_DIR/report.json"

echo "Visualizing results..."
cargo run --bin results_visualization -- --input "$WORK_DIR/report.json" > "$WORK_DIR/summary.txt"

echo "Simulating SDN integration..."
cargo run --bin sdn_controller_integration -- --model "$WORK_DIR/model.json" --output "$WORK_DIR/sdn_decision.json"

echo "Simulating NFV integration..."
cargo run --bin nfv_orchestrator_integration -- --model "$WORK_DIR/model.json" --output "$WORK_DIR/nfv_decision.json"

echo "Running real-time decision making..."
cargo run --bin real_time_decision_making -- --model "$WORK_DIR/model.json" --output "$WORK_DIR/realtime_decision.json" --rows 12

grep -q "Evaluation Summary" "$WORK_DIR/summary.txt"
grep -q "\"action\"" "$WORK_DIR/sdn_decision.json"
grep -q "\"action\"" "$WORK_DIR/nfv_decision.json"
grep -q "\"action\"" "$WORK_DIR/realtime_decision.json"

echo "Smoke test completed successfully."
