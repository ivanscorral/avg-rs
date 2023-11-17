use crate::traits::Average;

pub struct SimpleAverage;

impl Average<f64> for SimpleAverage {
    fn calculate(data: &[f64]) -> Result<f64, &'static str> {
        if data.is_empty() {
            return Err("data is empty");
        }
        let sum: f64 = data.iter().sum();
        Ok(sum / data.len() as f64)
    }
}
