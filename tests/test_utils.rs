extern crate rulinalg;
extern crate num as libnum;
extern crate rand;
extern crate rotex;

use rotex::utils::helper;
use rand::distributions::{Normal, IndependentSample};
use rotex::linalg::{Matrix, Vector};
//use rotex::layers::linear;
//use rotex::elem::{ElemType, ElemField, Elem};

#[test]
pub fn test_show_vector_f64() {

    let mut vec1 = Vector::<f64>::new(vec![1.0;20]);
    helper::show_vector_f64(&mut vec1);

}
