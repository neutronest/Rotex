use linalg::{Matrix, Vector};
use std::fmt;

pub fn show_vector_f64<T: fmt::Display>(vec: &mut Vector<T>) {

    print!("[");
    for x in (vec.data()) {
        print!("{},", x);
    }
    print!("]\n");
}

pub fn show_matrix_f64<T: fmt::Display> (mat: &mut Matrix<T>) {

    let row_num = mat.rows();
    let col_num = mat.cols();
    print!("[\n");
    for i in 0..row_num {
        print!("[");
        for j in 0..col_num {
            print!("{},", mat.data()[i*j]);
        }
        print!("]\n");
    }
    print!("]\n");

}
