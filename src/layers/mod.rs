use linalg::{Matrix, Vector}

pub mod sigmoid;
pub mod softmax;


pub trait Layer {

    fn forward(input_data: Vector<f64>) -> Vector<f64>;

    fn backward(input_data: Vector<f64>) -> Vector<f64>;
}



