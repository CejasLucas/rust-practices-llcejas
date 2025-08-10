use std::time::{Instant, Duration};
use crate::numerical_methods::interpolation_and_regression::strategy::InterpolationStrategy;
use crate::utils::format_input;

pub struct LagrangeInterpolation;

impl InterpolationStrategy for LagrangeInterpolation {
    fn name(&self) -> &str { "LAGRANGE" }

    fn execute(&self) -> Duration {
        self.print_header();
        let x_eval = format_input::read_f64("Enter the value of x: ");
        let start_time = Instant::now();
        Self::interpolate(&self, x_eval);
        start_time.elapsed()
    }
}

impl LagrangeInterpolation {
    fn interpolate(&self, x_eval: f64) -> f64 {
        let x = self.vector_of_x();
        let y = self.vector_of_y();
        let n = x.len();

        let mut result = 0.0;
        let mut symbolic_lines = Vec::new();
        let mut evaluated_terms = Vec::new();

        println!("\n📐 Lagrange Polynomial");
        println!("P(x) = Σ yᵢ * Lᵢ(x), where -> Lᵢ(x) = Π (x - xⱼ) / (xᵢ - xⱼ) for j ≠ i");
        
        for i in 0..n {
            let mut li_value = 1.0;
            let mut terms = Vec::new();

            for j in 0..n {
                if i != j {
                    let num = if x[j] < 0.0 {
                        format!("(x + {:.5})", -x[j])
                    } else {
                        format!("(x - {:.5})", x[j])
                    };

                    let den = x[i] - x[j];
                    let frac = format!("{}/ {:.5}", num, den);
                    terms.push(frac);

                    li_value *= (x_eval - x[j]) / den;
                }
            }

            let li_symbolic = terms.join(" * ");
            let li_full = format!("({})", li_symbolic);
            let term_val = y[i] * li_value;

            symbolic_lines.push(format!(
                "\nTerm {}: y[{}] * L_{}(x) =\n{:.5} * {}",
                i, i, i, y[i], li_full
            ));

            evaluated_terms.push(format!(
                "L_{}({:.5}) * y[{}] = {:.5} * {:.5} = {:.5}",
                i, x_eval, i, li_value, y[i], term_val
            ));

            result += term_val;
        }

        for line in symbolic_lines {
            println!("{}", line);
        }

        println!("\n🧮 Evaluated at x = {:.5}:", x_eval);
        for term in evaluated_terms {
            println!("{}", term);
        }

        println!("\n🎯 Lagrange Polynomial evaluated at x = {:.5}: P({:.5}) = {:.10}\n", x_eval, x_eval, result);
        result
    }
}