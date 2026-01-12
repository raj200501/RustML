pub mod linear_regression;
pub mod logistic_regression;

#[derive(Debug, Clone)]
pub struct TrainingReport {
    pub epochs: usize,
    pub final_loss: f64,
}

pub trait Model {
    fn train(&mut self, dataset: &crate::data::dataset::Dataset) -> TrainingReport;
    fn predict(&self, dataset: &crate::data::dataset::Dataset) -> Vec<f64>;
}
