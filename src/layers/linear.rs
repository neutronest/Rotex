//
//
//
use linalg::{Matrix, Vector};
use distributions::normal::Normal;

#[derive(debug)]
pub struct LinearLayer {
    
    input_size: int,
    output_size: int,
    input_data: Vector<f64>,
    output_data: Vector<f64>,
    weights: Matrix<f64>,
    bias: Vector<f64>
}

fn linearlayer_init(input_size_: int,
                    output_size_: int,
                    init_type_: String) -> LinearLayer {

    let mut linear_layer = {
        input_size: input_size_,
        output_size: output_size_,
        input_data: linglg::Vector::<f64>::zeros(output_size_),
        output_data: linalg::Vector::<f64>::zeros(output_size_),
        weights: match init_type_ {
            "normal" => {
                let normal = Normal
            },
            "ones" => linalg::Vector::<f64>::ones(input_size_),
            "zeros" | _ => linalg::Vector::<f64>::zeros(input_size_) 
        },
        bias: 


}

impl SimpleLayer for LinearLayer {

    fn layer_init() {

    }

} 

