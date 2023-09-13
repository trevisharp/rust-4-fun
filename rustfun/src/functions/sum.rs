use super::function::*;

pub struct Sum(Box<dyn Function>, Box<dyn Function>);

impl Function for Sum {
    fn calc(&self, x:f32) -> f32 {
        self.0.calc(x) + self.1.calc(x)
    }

    fn derive(&self) -> Box<dyn Function> {
        let f = self.0.derive();
        let g = self.1.derive();
        Box::new(Sum(f, g))
    }
}