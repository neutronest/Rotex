/*
extern crate rulinalg;
extern crate num as libnum;
extern crate rand;
extern crate rotex;

use rotex::utils::helper;
use rand::distributions::{Normal, IndependentSample};
use rotex::layers::linear;
use rotex::elem::{ElemType, Elem};


pub mod linalg {

    pub use rulinalg::matrix::{Axes, Matrix, MatrixSlice, MatrixSliceMut};
    pub use rulinalg::matrix::slice::BaseSlice;
    pub use rulinalg::vector::Vector;
    pub use rulinalg::Metric;
}


// ======== Layer Test ===========

// Linear Layer test


fn test_linearlayer_create() {

    let mut linear_layer = linear::linearlayer_init(10, 10, "normal");
    helper::show_matrix_f64(linear_layer.weights);
    helper::show_matrix_f64(linear_layer.bias);
}
*/
