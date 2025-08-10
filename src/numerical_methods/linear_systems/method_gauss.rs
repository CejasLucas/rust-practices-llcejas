use ndarray::{Array1};
use std::time::{Instant, Duration};
use crate::utils::format_arrays;
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
    pub fn final_resolution(&self) -> Array1<f64>{

        let mut a = Self::matrix(&self);
        let mut b = Self::vector(&self);
        let n = a.nrows();

        for k in 0..n {
            for i in (k + 1)..n {
                let factor = a[[i, k]] / a[[k, k]];
                for j in k..n {
                    a[[i, j]] -= factor * a[[k, j]];
                }
                b[i] -= factor * b[k];
            }
        }

        let mut x = Array1::<f64>::zeros(n);
        for i in (0..n).rev() {
            let mut sum = 0.0;
            for j in (i + 1)..n {
                sum += a[[i, j]] * x[j];
            }
            x[i] = (b[i] - sum) / a[[i, i]];
        }
        x
    }
}