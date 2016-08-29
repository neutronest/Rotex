
use linalg::{Vector, Matrix, Metric};


pub enum ElemType {

    Params,
    Nums,
    GParams
}

pub enum ElemField<T> {

    EMatrix(Matrix<T>),
    EVector(Vector<T>),
    ETensor4D
}

pub struct Elem<T> {
    
    pub elem_type: ElemType,

    pub elem_field: ElemField<T>

}

impl<T> Elem<T> where T: {

    pub fn new(elem_type_: ElemType, elem_field_: ElemField<T>) -> Elem<T> {

        Elem {
            elem_type: elem_type_,
            elem_field: elem_field_
        }
    }

    pub new_from_vector(elem_type_: ElemType, &mut vec: Vector<T>) {
        Elem {
            elem_type: elem_type_,
            elem_field: ElemField::EVector(vec)
        }
    }

    pub new_from_matrix(elem_type_: ElemType, &mut mat: Matrix<T>) {
        Elem {
            elem_type: elem_type_,
            elem_field: ElemField::EMatrix(mat)
        }
    }

    pub get_shape(&mut self) {
        match self.elem_field {

            vec: ElemField::Vector<T> => 

        }
    }
    
    pub fn show(&self) {

        match self.elem_type {

            ElemType::Params => {
                println!("type: params");
            },
            ElemType::Nums => {
                println!("type: nums");
            }
        };
        //TODO pritn elem_field
    }
}
