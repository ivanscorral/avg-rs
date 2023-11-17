// src/traits/average.rs

pub trait Average<T> {
    fn calculate(data: &[T]) -> Result<T, &'static str>;
}
