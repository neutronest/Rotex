extern crate rulinalg;
extern crate num as libnum;
extern crate rand;
//extern crate layers::{sigmoid, softmax};

pub mod linalg {

    pub use rulinalg::matrix::{Axes, Matrix, MatrixSlice, MatrixSliceMut};
    pub use rulinalg::matrix::slice::BaseSlice;
    pub use rulinalg::vector::Vector;
    pub use rulinalg::Metric;
}

#[test]
fn test_rulinalg_matrix() {

    println!("[Test test_rulinalg_matrix]")
}

fn test_rulinalg_vec() {

    let mut vec1: linalg::Vector<f64> = linalg::Vector::<f64>::new(vec![2.0; 20]);
    println!("{}", vec1);

}
