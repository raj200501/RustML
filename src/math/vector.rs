pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

pub fn subtract(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
}

pub fn scale(a: &[f64], factor: f64) -> Vec<f64> {
    a.iter().map(|x| x * factor).collect()
}

pub fn sigmoid(a: &[f64]) -> Vec<f64> {
    a.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect()
}

pub fn zeros(len: usize) -> Vec<f64> {
    vec![0.0; len]
}
