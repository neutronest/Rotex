// Author: neutronest
// Status: Developing
// the sigmoid layer

use linalg::{Matrix, Vector};

#[derive(Debug)]
pub struct SigmoidLayer{

    intput_data: Vector<f64>,
    output_data: Vector<f64>,
    weights: Matrix<f64>,
    bias: Vector<f64>
}

impl Layer for SigmoidLayer {

    fn forward(input_data: Vector<f64>) -> Vector<f64> {
        let Self::output_data = 

    }

    fn backward(end_data: Vector<f64>) -> Vector<f64> {

    }


}





