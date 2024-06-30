#[cfg(test)]
mod tests {
    use super::super::ml::linear_regression::LinearRegression;
    use super::super::data::dataset::Dataset;
    use super::super::utils::normalization::normalize;

    #[test]
    fn test_linear_regression() {
        let data = Dataset::from_csv("data/iris.csv").unwrap();
        let normalized_data = normalize(&data);
        
        let mut model = LinearRegression::new();
        model.train(&normalized_data);
        
        let predictions = model.predict(&normalized_data);
        assert_eq!(predictions.len(), data.data.len());
    }
}
