use csv::ReaderBuilder;
use std::error::Error;


pub fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .from_path(file_path)?;

let mut x_values = Vec::new();
let mut y_values = Vec::new();

for (i, result) in reader.records().enumerate() {
    match result {
        Ok(record) => {
            // println!("Record {}: {:?}", i + 1, record);
            let x: f64 = match record[0].parse() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error parsing 'x' at record {}: {}", i + 1, e);
                    continue;  // Skip this record
                }
            };
            let y: f64 = match record[1].parse() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error parsing 'y' at record {}: {}", i + 1, e);
                    continue;  // Skip this record
                }
            };
            x_values.push(x);
            y_values.push(y);
        }
        Err(e) => {
            eprintln!("Error reading record {}: {}", i + 1, e);
        }
    }
}

// println!("Parsed x values: {:?}", x_values);
// println!("Parsed y values: {:?}", y_values);

Ok((x_values, y_values))
}

pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

impl LinearRegression {
    pub fn new() -> Self {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    pub fn fit(&mut self, x: &[f64], y: &[f64]) {
        let n = x.len() as f64;

        let x_mean = x.iter().sum::<f64>() / n;
        let y_mean = y.iter().sum::<f64>() / n;

        let numerator = x.iter().zip(y.iter()).map(|(xi, yi)| (xi - x_mean) * (yi - y_mean)).sum::<f64>();
        let denominator = x.iter().map(|xi| (xi - x_mean).powi(2)).sum::<f64>();

        self.slope = numerator / denominator;
        self.intercept = y_mean - self.slope * x_mean;
    }

    // fn predict(&self, x: &[f64]) -> Vec<f64> {
    //     x.iter().map(|&xi| self.slope * xi + self.intercept).collect()
    // }
}