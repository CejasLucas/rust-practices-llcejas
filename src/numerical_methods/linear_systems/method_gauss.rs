use ndarray::{Array1};
use std::time::{Instant, Duration};
use crate::utils::{format_arrays, format_space};
use crate::{numerical_methods::linear_systems::strategy::LinearEquationSystemStrategy};
pub struct GaussMethod;

impl LinearEquationSystemStrategy for GaussMethod {
    fn name(&self) -> &str { "GAUSS" }

    fn execute(&self) -> Duration {
        self.print_header();
        self.print_equation();

        let start_time = Instant::now();
        let solution = self.final_resolution();
        println!("\n✅ Solution");

        println!("x = {}", format_arrays::vector_to_string_ndarray(&&solution));
        start_time.elapsed()
    }
}

impl GaussMethod {
    pub fn final_resolution(&self) -> Array1<f64> {
        let mut a = Self::matrix(&self);
        let mut b = Self::vector(&self);
        let n = a.nrows();

        println!("📋 Starting Gaussian Elimination");
        println!("🔢 Initial augmented matrix [A|b]:\n{}", format_arrays::augmented_matrix_to_string(&a, &b));

        // Eliminación hacia adelante
        for k in 0..n {
            println!();
            format_space::space("-", 100);
            println!("➡️ Pivot row k = {}", k);

            for i in (k + 1)..n {
                let factor = a[[i, k]] / a[[k, k]];
                println!(
                    "Eliminating A[{}, {}] using row {} → factor = A[{}, {}] / A[{}, {}] = {:.4} / {:.4} = {:.4}",
                    i, k, k, i, k, k, k, a[[i, k]], a[[k, k]], factor
                );

                for j in k..n {
                    let old_val = a[[i, j]];
                    a[[i, j]] -= factor * a[[k, j]];
                    println!(
                        "A[{}, {}] = {:.4} - ({:.4} * {:.4}) = {:.4}",
                        i, j, old_val, factor, a[[k, j]], a[[i, j]]
                    );
                }

                let old_b = b[i];
                b[i] -= factor * b[k];
                println!(
                    "b[{}] = {:.4} - ({:.4} * {:.4}) = {:.4}",
                    i, old_b, factor, b[k], b[i]
                );
            }

            println!("\n🔢 Augmented matrix after step k = {}:\n{}", k, format_arrays::augmented_matrix_to_string(&a, &b));
        }

        // Sustitución hacia atrás
        println!();
        format_space::space("-", 100);
        println!("⏪ Back Substitution");

        let mut x = Array1::<f64>::zeros(n);
        for i in (0..n).rev() {
            let mut sum = 0.0;
            for j in (i + 1)..n {
                sum += a[[i, j]] * x[j];
            }
            x[i] = (b[i] - sum) / a[[i, i]];
            println!(
                "x[{}] = (b[{}] - Σ A[{}, j]*x[j]) / A[{}, {}] = ({:.4} - {:.4}) / {:.4} = {:.4}",
                i, i, i, i, i, b[i], sum, a[[i, i]], x[i]
            );
        }

        x
    }
}