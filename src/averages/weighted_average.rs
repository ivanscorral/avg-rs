use crate::traits::Average;

#[derive(Debug, Copy, Clone)]
pub struct WeightedAverage {
    pub value: f64,
    pub weight: f64,
}
