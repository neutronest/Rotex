//
//
//
use linalg::{Matrix, Vector};
use distributions::{Normal, IndependentSample};
use act_fn;


#[derive(debug)]
pub struct LinearLayer {
    
    input_size: int,
    output_size: int,
    weights: Matrix<f64>,
    bias: Vector<f64>
}


pub fn linearlayer_init(input_size_: int,
                    output_size_: int,
                    init_type_: String) -> LinearLayer {

    let mut linear_layer = {
        input_size: input_size_,
        output_size: output_size_,
        weights: match init_type_ {
            "normal" => {
                let mut mat_ones = Matrix::new(input_size_,
                                               output_size_,
                                               vec![1.0; input_size_*output_size_]);
                let mut mat_normal = mat_ones.apply(&act_fn::act_std_normal);
                mat_normal
            },
            "ones" => Matrix::new(input_size_,
                                  output_size_,
                                  vec![1.0; input_size_ * output_size_]),
            "zeros" | _ => Matrix::new(input_size_,
                                       output_size_,
                                       vec![1.0; input_size_ * output_size_]) 
        },
        bias: linalg::Matrix::new(input_size_,
                                  output_size_,
                                  vec![0.0; input_size_*output_size_])

    };
    linear_layer
}
    


impl SimpleLayer for LinearLayer {

    fn forward(&mut self, input_data: Vector<f64>) -> Vector<f64> {
        self.weights * input_data + self.bias;
    }

    fn backward(&mut self, output_data_: Vector<f64>) -> Vector<f64> {
        self.weights
    }
} 

