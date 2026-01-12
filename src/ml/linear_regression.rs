use crate::data::dataset::Dataset;
use crate::math::vector;
use crate::ml::{Model, TrainingReport};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearRegression {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub learning_rate: f64,
    pub epochs: usize,
}

impl LinearRegression {
    pub fn new(num_features: usize) -> Self {
        LinearRegression {
            weights: vec![0.0; num_features],
            bias: 0.0,
            learning_rate: 0.001,
            epochs: 300,
        }
    }

    pub fn with_params(num_features: usize, learning_rate: f64, epochs: usize) -> Self {
        LinearRegression {
            weights: vec![0.0; num_features],
            bias: 0.0,
            learning_rate,
            epochs,
        }
    }

    fn predict_row(&self, row: &[f64]) -> f64 {
        vector::dot(row, &self.weights) + self.bias
    }

    fn compute_loss(&self, dataset: &Dataset) -> f64 {
        let predictions: Vec<f64> = dataset.data.iter().map(|row| self.predict_row(row)).collect();
        let n = dataset.target.len().max(1) as f64;
        predictions
            .iter()
            .zip(dataset.target.iter())
            .map(|(p, t)| (p - t).powi(2))
            .sum::<f64>()
            / n
    }
}

impl Model for LinearRegression {
    fn train(&mut self, dataset: &Dataset) -> TrainingReport {
        let n_samples = dataset.data.len() as f64;
        for _ in 0..self.epochs {
            let mut weight_gradients = vec![0.0; self.weights.len()];
            let mut bias_gradient = 0.0;
            for (row, target) in dataset.iter_rows() {
                let prediction = self.predict_row(row);
                let error = prediction - target;
                bias_gradient += error;
                for (idx, value) in row.iter().enumerate() {
                    weight_gradients[idx] += error * value;
                }
            }
            for (idx, weight) in self.weights.iter_mut().enumerate() {
                let gradient = weight_gradients[idx] / n_samples;
                let clipped = gradient.max(-1_000.0).min(1_000.0);
                *weight -= self.learning_rate * clipped;
            }
            let bias_grad = (bias_gradient / n_samples).max(-1_000.0).min(1_000.0);
            self.bias -= self.learning_rate * bias_grad;
        }
        TrainingReport {
            epochs: self.epochs,
            final_loss: self.compute_loss(dataset),
        }
    }

    fn predict(&self, dataset: &Dataset) -> Vec<f64> {
        dataset.data.iter().map(|row| self.predict_row(row)).collect()
    }
}
