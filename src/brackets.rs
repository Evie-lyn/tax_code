#[derive(Debug, Clone, Copy)]

pub struct Bracket(pub f64, pub f64); 
macro_rules! no_income_tax {
    () => {
        vec![Bracket(f64::INFINITY, 0.0)]
    };
}