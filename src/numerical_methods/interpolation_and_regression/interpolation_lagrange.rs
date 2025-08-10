use std::time::{Instant, Duration};
use crate::numerical_methods::interpolation_and_regression::strategy::InterpolationStrategy;
use crate::utils::format_input;

pub struct LagrangeInterpolation;

impl InterpolationStrategy for LagrangeInterpolation {
    fn name(&self) -> &str { "LAGRANGE" }

    // Executes the interpolation process and returns the elapsed time
    fn execute(&self) -> Duration {
        self.print_header();
        // Read the value where the interpolation is evaluated
        let x_eval = format_input::read_f64("Enter the value of a: ");
        let start_time = Instant::now();
        // Perform interpolation at x_eval
        Self::interpolate(&self, x_eval);
        // Return the elapsed time
        start_time.elapsed()
    }
}

impl LagrangeInterpolation {
    // Performs Lagrange interpolation at the given x_eval
    fn interpolate(&self, x_eval: f64) -> f64 {
        let x = self.vector_of_x();
        let y = self.vector_of_y();
        let n = x.len();

        let mut result = 0.0;
        // Loop over each term of the Lagrange polynomial
        for i in 0..n {
            let mut term = y[i];
            // Compute the Lagrange basis polynomial L_i(x_eval)
            for j in 0..n {
                if i != j {
                    term *= (x_eval - x[j]) / (x[i] - x[j]);
                }
            }
            println!("L_{}({}) * y = {}", i, x_eval, term);
            result += term;
        }

        // Print the final result of the interpolation
        println!("\n🎯 Lagrange Polynomial result P({}) = {}\n", x_eval, result);
        result
    }
}