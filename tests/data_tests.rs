#[cfg(test)]
mod tests {
    use super::super::data::dataset::Dataset;

    #[test]
    fn test_from_csv() {
        let dataset = Dataset::from_csv("data/iris.csv").unwrap();
        assert_eq!(dataset.data.len(), 150);
        assert_eq!(dataset.target.len(), 150);
    }
}
