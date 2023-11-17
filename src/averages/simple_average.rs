use crate::traits::Average;

pub struct SimpleAverage<'a> {
    data: &'a [f64],
}

impl<'a> SimpleAverage<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        Self { data }
    }
    
}

impl<'a> Average for SimpleAverage<'a> {
    
  
    fn calculate(&self) -> Result<f64, &'static str> {
        if self.data.is_empty() {
            return Err("data is empty");
        }
        let sum: f64 = self.data.iter().sum();
        Ok(sum / self.data.len() as f64)
    }
}
