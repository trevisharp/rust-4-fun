use super::function::*;

pub struct Constant(pub f32);

impl Function for Constant {
    fn calc(&self, _x:f32) -> f32 {
        self.0
    }

    fn derive(&self) -> Box<dyn Function> {
        Box::new(Constant(0.0))
    }
}