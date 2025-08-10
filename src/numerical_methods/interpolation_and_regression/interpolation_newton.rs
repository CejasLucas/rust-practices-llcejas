use std::time::{Instant, Duration};
use crate::numerical_methods::interpolation_and_regression::strategy::InterpolationStrategy;
use crate::utils::format_input;

pub struct NewtonInterpolation;

impl InterpolationStrategy for NewtonInterpolation {
    fn name(&self) -> &str { "NEWTON" }

    fn execute(&self) -> Duration {
        self.print_header();
        let x_eval = format_input::read_f64("Enter a test value for x: ");
        let start_time = Instant::now();
        Self::interpolate(&self, x_eval);
        start_time.elapsed()
    }
}

impl NewtonInterpolation {
    fn divided_differences(&self) -> Vec<Vec<f64>> {
        let x = self.vector_of_x();
        let y = self.vector_of_y();
        let n = x.len();
        let mut dd = vec![vec![0.0; n]; n];

        for i in 0..n {
            dd[i][0] = y[i];
        }

        for j in 1..n {
            for i in 0..(n - j) {
                dd[i][j] = (dd[i + 1][j - 1] - dd[i][j - 1]) / (x[i + j] - x[i]);
            }
        }

        dd
    }

    fn interpolate(&self, x_eval: f64) -> f64 {
        let x = self.vector_of_x();
        let n = x.len();
        let dd = self.divided_differences();

        let mut result = dd[0][0];
        let mut product_term = 1.0;

        let mut evaluated_terms = vec![format!("Term 0: {:.5}", dd[0][0])];
        let mut symbolic_lines = vec![format!("{:.5}", dd[0][0])];

        for i in 1..n {
            product_term *= x_eval - x[i - 1];
            let coeff = dd[0][i];
            let term_val = coeff * product_term;

            let sign = if coeff < 0.0 { "-" } else { "+" };
            let abs_coeff = coeff.abs();

            // Build term string with correct signs and parentheses
            let mut term_str = format!("{} {:.5}", sign, abs_coeff);
            for j in 0..i {
                if x[j] < 0.0 {
                    term_str += &format!("(x + {:.5})", -x[j]);
                } else {
                    term_str += &format!("(x - {:.5})", x[j]);
                }
            }

            symbolic_lines.push(term_str);
            evaluated_terms.push(format!("Term {}: {:.5}", i, term_val));
            result += term_val;
        }

        // Print symbolic polynomial vertically
        println!("\n📐 Newton Polynomial");
        println!("P(x) = f[x₀] + f[x₀, x₁](x - x₀) + f[x₀, x₁, x₂](x - x₀)(x - x₁) + ... + f[x₀, ..., xₙ](x - x₀)(x - x₁)...(x - xₙ₋₁)");
        for line in symbolic_lines {
            println!("{}", line);
        }

        // Print evaluated terms
        println!("\n🧮 Evaluated at x = {}:", x_eval);
        for term in evaluated_terms {
            println!("{}", term);
        }

        // Final result
        println!("\n🎯 Newton Polynomial result P({}) = {:.10}\n", x_eval, result);
        result
    }
}