// src/traits/average.rs

pub trait Average {
    fn calculate(&self) -> Result<f64, &'static str>;
}
