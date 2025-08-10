use std::time::{Instant, Duration};
use crate::utils::format_input;
use crate::numerical_methods::root_finding::strategy::NonlinearEquationsStrategy;
pub struct NewtonRaphsonMethod;

impl NonlinearEquationsStrategy for NewtonRaphsonMethod {
    fn name(&self) -> &str { "NEWTON-RAPHSON" }
    
    fn print_function(&self) -> &str { "f(x) = x³ - x - 2" }

    fn execute(&self) -> Duration {
        self.print_header();
        let x0 = format_input::read_f64("Enter initial guess (x0): ");
        let max_iter = format_input::read_u32("Enter maximum number of iterations: ");
        
        let start_time = Instant::now();
        Self::find_root_point(x0, max_iter);
        start_time.elapsed()
    }

}

impl NewtonRaphsonMethod {
    fn function(x: f64) -> f64 { x.powi(3) - x - 2.0 }

    fn function_derivative(x: f64) -> f64 { 3.0 * x.powi(2) - 1.0 }

    fn newton_raphson_step(x: f64, fx: f64) -> Option<f64> {
        let fpx = Self::function_derivative(x);

        if fpx.abs() < 1e-12 {
            println!("Derivative too close to zero at x = {:.6}. Method will not converge.", x);
            return None;
        }
        Some(x - fx / fpx)
    }

    fn find_root_point(mut x0: f64, max_iter: u32) -> Option<f64> {
        for iter in 0..max_iter {
            let fx = Self::function(x0);
            if let Some(x1) = Self::newton_raphson_step(x0, fx) {
                println!("Iteration {:>2}: x = {:>12.6}, f(x) = {:>12.6}", iter + 1, x1, Self::function(x1) );
                if (x1 - x0).abs() < Self.tolerance() { 
                    return Some(x1); 
                }
                x0 = x1;
            } else {
                return None;
            }
        }
        println!("Maximum iterations reached without convergence.");
        None
    }
}