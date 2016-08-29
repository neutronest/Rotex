
use linalg::{Vector, Matrix, Metric};
use std::fmt;
use utils::helper;

pub enum ElemType {

    Params,
    Nums,
    GParams
}

pub enum ElemField<T> {

    EMatrix{emat: Matrix<T>},
    EVector{evec: Vector<T>},
    ETensor4D
}

pub struct Elem<T> {
    
    pub elem_type: ElemType,

    pub elem_field: ElemField<T>

}

impl<T: fmt::Display> Elem<T>  {

    pub fn new(elem_type_: ElemType, elem_field_: ElemField<T>) -> Elem<T> {

        Elem {
            elem_type: elem_type_,
            elem_field: elem_field_
        }
    }

    pub fn new_from_vector(elem_type_: ElemType, mut vec: Vector<T>) -> Elem<T> {
        Elem {
            elem_type: elem_type_,
            elem_field: ElemField::EVector{evec: vec}
        }
    }

    pub fn new_from_matrix(elem_type_: ElemType, mut mat: Matrix<T>) -> Elem<T>{
        Elem {
            elem_type: elem_type_,
            elem_field: ElemField::EMatrix{emat: mat}
        }
    }

    /*
    pub get_shape(&mut self) {
        match self.elem_field {

            vec: ElemField::Vector<T> => 

        }
    }
     */


    pub fn show(&mut self) {

        match self.elem_type {

            ElemType::Params => {
                println!("type: params");
            },
            ElemType::Nums => {
                println!("type: nums");
            },
            ElemType::GParams => {
                println!("type: gparams");
            }
        };
        match self.elem_field {
            ElemField::EVector{ ref mut evec } => {
                helper::show_vector_f64(evec);
            },

            ElemField::EMatrix{ ref mut emat }  => {
                helper::show_matrix_f64(emat);
            },

            ElemField::ETensor4D => {}

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
