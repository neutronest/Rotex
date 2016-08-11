extern crate rulinalg;
extern crate num as libnum;
extern crate rand;
extern crate rotex;
//extern crate layers::{sigmoid, softmax};
use rotex::utils::helper;
use rand::distributions::{Normal, IndependentSample};

pub mod linalg {

    pub use rulinalg::matrix::{Axes, Matrix, MatrixSlice, MatrixSliceMut};
    pub use rulinalg::matrix::slice::BaseSlice;
    pub use rulinalg::vector::Vector;
    pub use rulinalg::Metric;
}

#[test]
fn test_rulinalg_matrix() {

    println!("[Test test_rulinalg_matrix]");
    let mut mat1: linalg::Matrix<f64> = linalg::Matrix::<f64>::new(4, 5, vec![2.0; 20]);
    helper::show_matrix_f64(mat1);

}

#[test]
fn test_rulinalg_vec() {

    let mut vec1: linalg::Vector<f64> = linalg::Vector::<f64>::new(vec![2.0; 20]);
    helper::show_vector_f64(vec1);
}

#[test]
fn test_normal_distribution() {
    
    println!("[Test gen normal distribution]");
    let normal = Normal::new(0.0, 1.0);
    for i in 0..10 {
        let v = normal.ind_sample(&mut rand::thread_rng());
        println!("{}, ", v);
    }

}

#[test]
fn test_apply_normal_matrix() {

    let mut mat1: linalg::Matrix<f64> = linalg::Matrix::<f64>::new(20, 20, vec![1.0; 400]);
    //let normal = Normal::new(0.0, 1.0);
    fn normal_fn(v: f64) -> f64 {
        let normal = Normal::new(0.0, 1.0);
        normal.ind_sample(&mut rand::thread_rng())
    };
    let mat1 = mat1.apply(&normal_fn);
    helper::show_matrix_f64(mat1);

}
