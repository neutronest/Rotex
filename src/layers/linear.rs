//
//
//

use linalg::{Matrix, Vector};
use rand::distributions::{Normal, IndependentSample};
use act_fn;
use layers::{SimpleLayer};

//#[derive(debug)]
pub struct LinearLayer {
    
    pub input_size: usize,
    pub output_size: usize,
    pub weights: Matrix<f64>,
    pub bias: Vector<f64>
}


pub fn linearlayer_init(input_size_: usize,
                        output_size_: usize,
                        init_type_: String) -> LinearLayer {

    let mut linear_layer = LinearLayer {
        input_size: input_size_,
        output_size: output_size_,
        weights: match init_type_.as_ref() {
            "normal" => {
                let mut mat_ones = Matrix::<f64>::new(input_size_,
                                                      output_size_,
                                                      vec![1.0; input_size_*output_size_]);
                let mut mat_normal = mat_ones.apply(&act_fn::act_std_normal);
                mat_normal
            },
            "one" => Matrix::<f64>::new(input_size_,
                                         output_size_,
                                         vec![1.0; input_size_ * output_size_]),
            "zeros" | _ => Matrix::<f64>::new(input_size_,
                                              output_size_,
                                              vec![1.0; input_size_ * output_size_]) 
        },
        bias: Vector::<f64>::new(vec![0.0; output_size_])
    };
    linear_layer
}
    


impl SimpleLayer for LinearLayer {

    fn forward(&mut self, input_data: Vector<f64>) -> Vector<f64> {
        assert_eq!(input_data.size(), self.weights.rows());

        // return
        self.weights * input_data  + self.bias
    }

 
    fn backward(&mut self, output_data_: Vector<f64>) -> Vector<f64> {

        // return
        panic!()
    }
}

