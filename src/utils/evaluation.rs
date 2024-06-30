pub fn mean_squared_error(predictions: &[f64], targets: &[f64]) -> f64 {
    let n = predictions.len();
    predictions.iter().zip(targets).map(|(p, t)| (p - t).powi(2)).sum::<f64>() / n as f64
}
