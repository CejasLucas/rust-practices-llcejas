use std::time::{Instant, Duration};
use crate::utils::format_input;
use crate::numerical_methods::nonlinear_equations::NonlinearEquationsStrategy;
pub struct SecantMethod;

impl NonlinearEquationsStrategy for SecantMethod {
    fn name(&self) -> &str { "SECANT" }

    fn print_function(&self) -> &str { "f(x) = x³ - x - 1" }

    fn execute(&self) -> Duration {
        self.print_header();
        let x0 = format_input::read_f64("Enter first initial guess (x0): ");
        let x1 = format_input::read_f64("Enter second initial guess (x1): ");
        let max_iter = format_input::read_u32("Enter maximum number of iterations: ");
        
        let start_time = Instant::now();
        Self::find_root_point(x0, x1, max_iter);
        start_time.elapsed()
    }
}

impl SecantMethod {
    fn function(x: f64) -> f64 { x.powi(3) - x - 1.0 }

    fn find_root_point(mut x0: f64, mut x1: f64, max_iter: u32) -> Option<f64> {
        for iter in 0..max_iter {
            let fx0 = Self::function(x0);
            let fx1 = Self::function(x1);

            if (fx1 - fx0).abs() < 1e-12 {
                println!("⚠️ Denominator too close to zero at iteration {}. Method may not converge.", iter + 1);
                return None;
            }

            let x2 = x1 - fx1 * (x1 - x0) / (fx1 - fx0);
            let fx2 = Self::function(x2);
            println!("Iteration {:>2}: x = {:>12.6}, f(x) = {:>12.6}", iter + 1, x2, fx2);

            if (x2 - x1).abs() < Self.tolerance() {
                return Some(x2);
            }
            x0 = x1;
            x1 = x2;
        }

        println!("Maximum number of iterations reached without convergence.");
        None
    }
}