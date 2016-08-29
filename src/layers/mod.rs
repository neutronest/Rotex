//extern crate rotex;
use linalg::{Matrix, Vector};
use rand::distributions;
use elem::Elem;

pub mod linear;
pub mod sigmoid;
pub mod softmax;

pub trait SimpleLayer<T> {

    fn forward(&mut self, input_data: Elem<T>) -> Elem<T>;

    fn backward(&mut self, input_data: Elem<T>) -> Elem<T>;
}



