use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dataset {
    pub feature_names: Vec<String>,
    pub target_name: String,
    pub data: Vec<Vec<f64>>,
    pub target: Vec<f64>,
}

#[derive(Debug)]
pub enum DatasetError {
    EmptyDataset,
    MismatchedRowLengths { expected: usize, found: usize },
    MissingTarget,
    InvalidColumnCount { expected: usize, found: usize },
    InvalidTargetCount { expected: usize, found: usize },
}

impl fmt::Display for DatasetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatasetError::EmptyDataset => write!(f, "dataset is empty"),
            DatasetError::MismatchedRowLengths { expected, found } => write!(
                f,
                "row length mismatch: expected {expected} columns, found {found}"
            ),
            DatasetError::MissingTarget => write!(f, "target column is missing"),
            DatasetError::InvalidColumnCount { expected, found } => write!(
                f,
                "invalid column count: expected {expected} columns, found {found}"
            ),
            DatasetError::InvalidTargetCount { expected, found } => write!(
                f,
                "invalid target count: expected {expected} targets, found {found}"
            ),
        }
    }
}

impl Error for DatasetError {}

impl Dataset {
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(path)?;
        let headers = reader
            .headers()?
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<String>>();

        if headers.len() < 2 {
            return Err(Box::new(DatasetError::MissingTarget));
        }

        let feature_names = headers[..headers.len() - 1].to_vec();
        let target_name = headers.last().cloned().unwrap_or_else(|| "target".to_string());

        let mut data = Vec::new();
        let mut target = Vec::new();

        for result in reader.records() {
            let record = result?;
            if record.len() != headers.len() {
                return Err(Box::new(DatasetError::InvalidColumnCount {
                    expected: headers.len(),
                    found: record.len(),
                }));
            }
            let mut row = Vec::with_capacity(feature_names.len());
            for (idx, field) in record.iter().enumerate() {
                let value: f64 = field.parse()?;
                if idx == headers.len() - 1 {
                    target.push(value);
                } else {
                    row.push(value);
                }
            }
            data.push(row);
        }

        if data.is_empty() {
            return Err(Box::new(DatasetError::EmptyDataset));
        }

        Ok(Dataset {
            feature_names,
            target_name,
            data,
            target,
        })
    }

    pub fn from_records(
        feature_names: Vec<String>,
        target_name: impl Into<String>,
        data: Vec<Vec<f64>>,
        target: Vec<f64>,
    ) -> Result<Self, DatasetError> {
        if data.is_empty() {
            return Err(DatasetError::EmptyDataset);
        }
        let expected_len = data[0].len();
        for row in &data {
            if row.len() != expected_len {
                return Err(DatasetError::MismatchedRowLengths {
                    expected: expected_len,
                    found: row.len(),
                });
            }
        }
        if target.len() != data.len() {
            return Err(DatasetError::InvalidTargetCount {
                expected: data.len(),
                found: target.len(),
            });
        }
        Ok(Dataset {
            feature_names,
            target_name: target_name.into(),
            data,
            target,
        })
    }

    pub fn to_csv(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::WriterBuilder::new().has_headers(true).from_path(path)?;
        let mut headers = self.feature_names.clone();
        headers.push(self.target_name.clone());
        writer.write_record(headers)?;
        for (row, target_value) in self.data.iter().zip(self.target.iter()) {
            let mut record = Vec::with_capacity(row.len() + 1);
            for value in row {
                record.push(value.to_string());
            }
            record.push(target_value.to_string());
            writer.write_record(&record)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn num_features(&self) -> usize {
        self.data.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn feature_names(&self) -> &[String] {
        &self.feature_names
    }

    pub fn targets(&self) -> &[f64] {
        &self.target
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = (&[f64], f64)> {
        self.data
            .iter()
            .zip(self.target.iter())
            .map(|(row, target)| (row.as_slice(), *target))
    }

    pub fn add_feature(&mut self, name: impl Into<String>, values: Vec<f64>) -> Result<(), DatasetError> {
        if values.len() != self.data.len() {
            return Err(DatasetError::InvalidTargetCount {
                expected: self.data.len(),
                found: values.len(),
            });
        }
        for (row, value) in self.data.iter_mut().zip(values.into_iter()) {
            row.push(value);
        }
        self.feature_names.push(name.into());
        Ok(())
    }

    pub fn map_features<F>(&self, mut func: F) -> Dataset
    where
        F: FnMut(&[f64]) -> Vec<f64>,
    {
        let data = self.data.iter().map(|row| func(row)).collect();
        Dataset {
            feature_names: self.feature_names.clone(),
            target_name: self.target_name.clone(),
            data,
            target: self.target.clone(),
        }
    }

    pub fn subset(&self, indices: &[usize]) -> Result<Dataset, DatasetError> {
        if indices.is_empty() {
            return Err(DatasetError::EmptyDataset);
        }
        let mut data = Vec::with_capacity(indices.len());
        let mut target = Vec::with_capacity(indices.len());
        for &index in indices {
            if index >= self.data.len() {
                return Err(DatasetError::InvalidTargetCount {
                    expected: self.data.len(),
                    found: index,
                });
            }
            data.push(self.data[index].clone());
            target.push(self.target[index]);
        }
        Ok(Dataset {
            feature_names: self.feature_names.clone(),
            target_name: self.target_name.clone(),
            data,
            target,
        })
    }

    pub fn train_test_split(&self, test_ratio: f64) -> Result<(Dataset, Dataset), DatasetError> {
        if !(0.0..1.0).contains(&test_ratio) {
            return Err(DatasetError::InvalidColumnCount {
                expected: 1,
                found: 0,
            });
        }
        let test_size = ((self.data.len() as f64) * test_ratio).round() as usize;
        let test_indices: Vec<usize> = (0..test_size).collect();
        let train_indices: Vec<usize> = (test_size..self.data.len()).collect();
        Ok((self.subset(&train_indices)?, self.subset(&test_indices)?))
    }
}
