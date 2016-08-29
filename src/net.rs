// the net basic architecture


extern crate rotex;

use rotex::layers;
use rotes::elem;

pub struct Net<T> {

    name: String,

    layers: Vec<layers::Layer>,

    params: Vec<elem::Elem<T>>
}
