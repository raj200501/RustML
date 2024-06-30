use rustml::data::dataset::Dataset;
use rustml::ml::linear_regression::LinearRegression;
use rustml::utils::normalization::normalize;

fn main() {
    let data = Dataset::from_csv("data/iris.csv").unwrap();
    let normalized_data = normalize(&data);
    
    let mut model = LinearRegression::new();
    model.train(&normalized_data);
    
    let predictions = model.predict(&normalized_data);
    println!("Predictions: {:?}", predictions);
}
