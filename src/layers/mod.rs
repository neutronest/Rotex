use linalg::{Matrix, Vector};
use rand::distributions;
pub mod sigmoid;
pub mod softmax;


pub trait SimpleLayer {

    fn forward(input_data: Vector<f64>) -> Vector<f64>;

    fn backward(input_data: Vector<f64>) -> Vector<f64>;
}



