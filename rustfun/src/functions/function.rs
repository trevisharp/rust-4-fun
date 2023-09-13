use super::{sum::Sum, constant::Constant, linear::Linear};

pub trait Function {

    fn calc(&self, x:f32) -> f32;
    fn derive(&self) -> Box<dyn Function>;
}

pub fn cons(value: f32) -> Box<Constant> {
    Box::new(Constant(value))
}

pub fn x() -> Box<Linear> {
    Box::new(Linear)
}

pub fn sum(f: Box<dyn Function>, g: Box<dyn Function>) -> Sum
{
    Sum(f, g)
}