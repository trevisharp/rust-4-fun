pub mod functions;

use crate::functions::function::*;

fn main() {
    let f = sum(cons(4.0), x());
    let g = f.derive();
    let y = g.calc(10.0);
    println!("{y}");
}