
use linalg::{Vector, Matrix, Metric};
use std::fmt;
use utils::helper;


pub enum Elem<T> {

    EVector{edata: Vector<T>},
    EMatrix{edata: Matrix<T>},
    ETensor4D
}


impl<T: fmt::Display> Elem<T>  {

    

    pub fn new_from_vector(mut vec: Vector<T>) -> Elem<T> {
        Elem::EVector{edata: vec}
    }

    pub fn new_from_matrix(mut mat: Matrix<T>) -> Elem<T>{
        Elem::EMatrix{edata: mat}
    }

    pub fn show(&mut self) {

        
        match &mut self {
            Elem::EVector{edata: vec} => {
                helper::show_vector(ref mut vec);
            },

            Elem::EMatrix {edata: mat} => {
                helper::show_matrix(ref mut mat);
            },
            Elem::ETensor4D => {}
        }
    }
}

/*
impl<T: fmt::Display>  fmt::Display for Elem<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.elem_type {

            ElemType::Params => {
                write!(f, "type: params");
            },
            ElemType::Nums => {
                write!(f, "type: nums");
            },
            ElemType::GParams => {
                write!(f, "type: gparams");
            }
        };

    }

}
*/
