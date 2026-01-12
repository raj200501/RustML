use crate::data::dataset::Dataset;
use crate::math::vector;
use crate::ml::{Model, TrainingReport};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticRegression {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub learning_rate: f64,
    pub epochs: usize,
}

impl LogisticRegression {
    pub fn new(num_features: usize) -> Self {
        LogisticRegression {
            weights: vec![0.0; num_features],
            bias: 0.0,
            learning_rate: 0.05,
            epochs: 300,
        }
    }

    fn sigmoid(value: f64) -> f64 {
        1.0 / (1.0 + (-value).exp())
    }

    fn predict_probability(&self, row: &[f64]) -> f64 {
        let linear = vector::dot(row, &self.weights) + self.bias;
        Self::sigmoid(linear)
    }

    fn compute_loss(&self, dataset: &Dataset) -> f64 {
        let epsilon = 1e-12;
        let mut loss = 0.0;
        for (row, target) in dataset.iter_rows() {
            let prediction = self.predict_probability(row);
            loss += -target * (prediction + epsilon).ln()
                - (1.0 - target) * (1.0 - prediction + epsilon).ln();
        }
        loss / dataset.data.len().max(1) as f64
    }
}

impl Model for LogisticRegression {
    fn train(&mut self, dataset: &Dataset) -> TrainingReport {
        let n_samples = dataset.data.len() as f64;
        for _ in 0..self.epochs {
            let mut weight_gradients = vec![0.0; self.weights.len()];
            let mut bias_gradient = 0.0;
            for (row, target) in dataset.iter_rows() {
                let prediction = self.predict_probability(row);
                let error = prediction - target;
                bias_gradient += error;
                for (idx, value) in row.iter().enumerate() {
                    weight_gradients[idx] += error * value;
                }
            }
            for (idx, weight) in self.weights.iter_mut().enumerate() {
                *weight -= self.learning_rate * (weight_gradients[idx] / n_samples);
            }
            self.bias -= self.learning_rate * (bias_gradient / n_samples);
        }
        TrainingReport {
            epochs: self.epochs,
            final_loss: self.compute_loss(dataset),
        }
    }

    fn predict(&self, dataset: &Dataset) -> Vec<f64> {
        dataset
            .data
            .iter()
            .map(|row| self.predict_probability(row))
            .collect()
    }
}
