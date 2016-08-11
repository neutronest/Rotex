extern crate rulinalg;
extern crate num as libnum;
//extern crate rand;
//extern crate layers::{sigmoid, softmax};

pub mod linalg {

    pub use rulinalg::matrix::{Axes, Matrix, MatrixSlice, MatrixSliceMut};
    pub use rulinalg::matrix::slice::BaseSlice;
    pub use rulinalg::vector::Vector;
    pub use rulinalg::Metric;
}

pub mod helper;

