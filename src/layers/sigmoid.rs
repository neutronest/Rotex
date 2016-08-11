// Author: neutronest
// Status: Developing
// the sigmoid layer
use linalg::{Matrix, Vector};
use layers::SimpleLayer;
#[derive(Debug)]
pub struct SigmoidLayer{

    intput_data: Vector<f64>,
    output_data: Vector<f64>,
    weights: Matrix<f64>,
    bias: Vector<f64>
}

/*
impl SimpleLayer for SigmoidLayer {

    fn forward(&mut self, input_data: Vector<f64>) -> Vector<f64> {

    }

    fn backward(&mut self, output_data: Vector<f64>) -> Vector<f64> {

    }


}
*/




