// This code is come from rust-manchine
// Author: 

pub trait ActivationFunc {
    /// The activation function.
    fn act_func(x: f64) -> f64;

    /// The gradient of the activation function.
    fn act_grad(x: f64) -> f64;

    /// The inverse of the activation function.
    fn act_inv(x: f64) -> f64;

}

/// Sigmoid activation function.
#[derive(Clone, Copy, Debug)]
pub struct Sigmoid;

impl ActivationFunc for Sigmoid {

    ///
    ///
    ///
    fn act_func(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Gradient of sigmoid function
    ///
    /// Evaluates to
    fn act_grad(x: f64) -> f64 {
        Self::act_func(x) * (1f64 - Self::act_func(x))
    }

    ///
    fn act_inv(x: f64) -> f64 {

        (x / (1f64 - x)).ln()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Linear;

impl ActivationFuncfor Linear {

    ///
    fn act_func(x: f64) -> f64 {
        

    }
}
