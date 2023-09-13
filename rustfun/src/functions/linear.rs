use super::function::*;
use super::constant::*;

pub struct Linear;

impl Function for Linear {
    fn calc(&self, x:f32) -> f32 {
        x
    }

    fn derive(&self) -> Box<dyn Function> {
        Box::new(Constant(1.0))
    }
}