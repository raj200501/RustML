use rustml::data::dataset::Dataset;
use rustml::pipeline::train_linear_pipeline;

fn main() {
    let dataset = Dataset::from_csv("data/iris.csv").expect("failed to load dataset");
    let result = train_linear_pipeline(&dataset).expect("failed to train pipeline");
    println!(
        "Training summary: mse={:.4}, rmse={:.4}, mae={:.4}, loss={:.4}, epochs={}",
        result.summary.mse,
        result.summary.rmse,
        result.summary.mae,
        result.summary.loss,
        result.summary.epochs
    );
}
