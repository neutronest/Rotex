//extern crate rotex;
use linalg::{Matrix, Vector};
use rand::distributions;
use elem;
use elem::{Elem};
pub mod linear;
pub mod sigmoid;
pub mod softmax;

use std::fmt;

pub trait SimpleLayer<T: fmt::Display> {

    fn forward(&mut self, input_data: Option<Elem<T>>) -> Option<Elem<T>>;

    fn backward(&mut self, input_data: Option<Elem<T>>) -> Option<Elem<T>>;
}



