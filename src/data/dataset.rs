use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;

pub struct Dataset {
    pub data: Vec<Vec<f64>>,
    pub target: Vec<f64>,
}

impl Dataset {
    pub fn from_csv(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;
        
        let mut data = Vec::new();
        let mut target = Vec::new();
        
        for result in rdr.records() {
            let record = result?;
            let mut row = Vec::new();
            for (i, field) in record.iter().enumerate() {
                if i == record.len() - 1 {
                    target.push(field.parse::<f64>()?);
                } else {
                    row.push(field.parse::<f64>()?);
                }
            }
            data.push(row);
        }
        
        Ok(Dataset { data, target })
    }
}
