pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mu = mean(values);
    values.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / values.len() as f64
}

pub fn std_dev(values: &[f64]) -> f64 {
    variance(values).sqrt()
}

pub fn min(values: &[f64]) -> f64 {
    values
        .iter()
        .cloned()
        .fold(f64::INFINITY, |acc, value| acc.min(value))
}

pub fn max(values: &[f64]) -> f64 {
    values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |acc, value| acc.max(value))
}
