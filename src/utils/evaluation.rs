#[derive(Debug, Clone)]
pub struct ConfusionMatrix {
    pub true_positive: usize,
    pub true_negative: usize,
    pub false_positive: usize,
    pub false_negative: usize,
}

impl ConfusionMatrix {
    pub fn accuracy(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            return 0.0;
        }
        (self.true_positive + self.true_negative) as f64 / total as f64
    }

    pub fn precision(&self) -> f64 {
        let denom = self.true_positive + self.false_positive;
        if denom == 0 {
            return 0.0;
        }
        self.true_positive as f64 / denom as f64
    }

    pub fn recall(&self) -> f64 {
        let denom = self.true_positive + self.false_negative;
        if denom == 0 {
            return 0.0;
        }
        self.true_positive as f64 / denom as f64
    }

    pub fn f1(&self) -> f64 {
        let precision = self.precision();
        let recall = self.recall();
        if precision + recall == 0.0 {
            return 0.0;
        }
        2.0 * precision * recall / (precision + recall)
    }

    pub fn total(&self) -> usize {
        self.true_positive + self.true_negative + self.false_positive + self.false_negative
    }
}

pub fn mean_squared_error(predictions: &[f64], targets: &[f64]) -> f64 {
    let n = predictions.len().max(1);
    predictions
        .iter()
        .zip(targets.iter())
        .map(|(p, t)| (p - t).powi(2))
        .sum::<f64>()
        / n as f64
}

pub fn root_mean_squared_error(predictions: &[f64], targets: &[f64]) -> f64 {
    mean_squared_error(predictions, targets).sqrt()
}

pub fn mean_absolute_error(predictions: &[f64], targets: &[f64]) -> f64 {
    let n = predictions.len().max(1);
    predictions
        .iter()
        .zip(targets.iter())
        .map(|(p, t)| (p - t).abs())
        .sum::<f64>()
        / n as f64
}

pub fn confusion_matrix(predictions: &[f64], targets: &[f64], threshold: f64) -> ConfusionMatrix {
    let mut matrix = ConfusionMatrix {
        true_positive: 0,
        true_negative: 0,
        false_positive: 0,
        false_negative: 0,
    };
    for (&prediction, &target) in predictions.iter().zip(targets.iter()) {
        let predicted_label = prediction >= threshold;
        let actual_label = target >= threshold;
        match (predicted_label, actual_label) {
            (true, true) => matrix.true_positive += 1,
            (false, false) => matrix.true_negative += 1,
            (true, false) => matrix.false_positive += 1,
            (false, true) => matrix.false_negative += 1,
        }
    }
    matrix
}
