extern crate rulinalg;
extern crate num as libnum;
extern crate rand;
extern crate rotex;

use rotex::utils::helper;
use rand::distributions::{Normal, IndependentSample};
//use rotex::layers::linear;
use rotex::elem::{Elem};


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
    let mut elem_a = Elem::EVector{edata: vec_a};
    elem_a.show();

}

#[test]
fn test_elem_new_from_matrix() {

    println!("[Test elem_new_from_matrix]");
    let mut elem_b = Elem::EMatrix{
        edata: linalg::Matrix::<f64>::new(4,5, vec![2.0; 20])
    }
    elem_b.show();

}
