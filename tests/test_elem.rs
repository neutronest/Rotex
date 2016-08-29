extern crate rulinalg;
extern crate num as libnum;
extern crate rand;
extern crate rotex;

use rotex::utils::helper;
use rand::distributions::{Normal, IndependentSample};
//use rotex::layers::linear;
use rotex::elem::{ElemType, ElemField, Elem};


pub mod linalg {

    pub use rulinalg::matrix::{Axes, Matrix, MatrixSlice, MatrixSliceMut};
    pub use rulinalg::matrix::slice::BaseSlice;
    pub use rulinalg::vector::Vector;
    pub use rulinalg::Metric;
}

#[test]
fn test_elem_new_from_vector() {

    println!("[Test elem_new_from_vector]");
    let mut vec_a = linalg::Vector::<f64>::new(vec![2.0; 20]);
    let mut vec_b = vec_a;
    helper::show_vector_f64(&mut vec_b);


    let mut elem_a = Elem::<f64>::new(ElemType::Nums, ElemField::EVector(&vec_b));

}
