//
//
//
use linalg::{Matrix, Vector};
use rand::distributions::{Normal, IndependentSample};
use act_fn;
use layers::{SimpleLayer};
use elem::{ElemType, ElemField, Elem};


pub struct LinearLayer<f64> {

    pub m_size: usize, // input size
    pub n_size: usize, // usize
    pub params: Elem<f64>, // elem with param type
    pub bias: Elem<f64>, // elem with param type
}

pub fn linearlayer_init(m_size_: usize,
                        n_size_: usize,
                        params_: Elem<f64>,
                        bias_: Elem<f64>) {

    LinearLayer {
        m_size: m_size_,
        n_size: n_size_,
        params: params_,
        bias: bias_
    }
}

impl SimpleLayer<f64> for LinearLayer<f64> {

    fn forward(&mut self, input_data: Option<Elem<f64>>) -> Option<Elem<f64>> {

        assert_eq!(self.params.elem_type, ElemType::Params);
        assert_eq!(self.params.elem_field.etype, "matrix".to_string());
        assert_eq!(self.bias.elem_type, ElemType::Params);
        assert_eq!(self.bias.elem_field.etype, "vector".to_string());

        
        None
        
    }

    fn backward(&mut self, input_data: Option<Elem<f64>>) -> Option<Elem<f64>> {
        None
    }

}

/*
pub fn linearlayer_init(m_size_ : usize,
                        n_size_ : usize,
                        params_init_type: String) {

    LinearLayer {

        m_size: m_size_,
        n_size: n_size_,
        params: {
            let mut params_mat = match params_init_type.as_ref() {
                "normal" => {
                    let mut mat_ones = Matrix::<f64>::new(m_size_,
                                                          n_size_,
                                                          vec![1.0; m_size_* n_size_]);
                    let mut mat_normal = mat_ones.apply(&act_fn::act_std_normal);
                    mat_normal

                }, // end of normal mat init
                "one" => {
                    Matrix::<f64>::new(m_size_,
                                       n_size,
                                       vec![1.0; m_size_*n_size_])
                },
                "zero" => {
                    Matrix::<f64>::new(m_size_,
                                       n_size,
                                       vec![0.0; m_size_*n_size_])
                }
            } // end of params_mat init
            Elem(ElemType::Params, ElemField::EMatrix(&params_mat))
        } // end of params init

    } // end of linear layer init

}
*/

/*
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
    


impl SimpleLayer<f64> for LinearLayer {

    fn forward(mut self, input_data: Vector<f64>) -> Vector<f64> {
        assert_eq!(input_data.size(), self.weights.rows());

        // return
        self.weights * input_data  + self.bias
    }

 
    fn backward(mut self, output_data_: Vector<f64>) -> Vector<f64> {

        // return
        panic!()
    }
}
*/
