pub trait Function {
    fn calc(&self, x:f32) -> f32;
    fn derive(&self) -> dyn Function;
}