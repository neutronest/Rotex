//
// Author: neutronest
extern crate rand;
use rand::distributions::{Normal, IndependentSample};


fn act_std_normal(v: f64) -> f64 {

    let normal = Normal::new(0.0, 1.0);
    // return
    normal.ind_sample(&mut rand::thread_rng())
}

fn act_sigmoid(v: f64) -> f64 {
    // return
    1.0 / (1.0 + (-x).exp())
}

fn act_sigmiod_grad(v: f64) -> f64{
    // return
    act_sigmoid(v) * (1f64 - act_sigmoid(v))
}
