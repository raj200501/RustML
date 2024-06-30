use super::super::data::dataset::Dataset;

pub fn normalize(dataset: &Dataset) -> Dataset {
    let mut normalized_data = dataset.data.clone();
    
    for j in 0..dataset.data[0].len() {
        let col: Vec<f64> = dataset.data.iter().map(|row| row[j]).collect();
        let mean: f64 = col.iter().sum::<f64>() / col.len() as f64;
        let std_dev: f64 = (col.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / col.len() as f64).sqrt();
        
        for i in 0..dataset.data.len() {
            normalized_data[i][j] = (dataset.data[i][j] - mean) / std_dev;
        }
    }
    
    Dataset {
        data: normalized_data,
        target: dataset.target.clone(),
    }
}
