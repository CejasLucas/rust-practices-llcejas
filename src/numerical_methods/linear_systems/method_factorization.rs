use ndarray::Array2;
use std::time::{Instant, Duration};
use crate::utils::{format_arrays, format_space};
use crate::{numerical_methods::linear_systems::strategy::LinearEquationSystemStrategy};

pub struct FactorizationLUMethod;

impl LinearEquationSystemStrategy for FactorizationLUMethod {
    fn name(&self) -> &str { "FACTORIZATION LU" }

    fn execute(&self) -> Duration {
        self.print_header();
        self.print_equation();

        let start_time = Instant::now();
        let (l, u) = self.final_resolution();

        println!("🧮 Factorization completed. Final Result:");
        println!("L =\n{}", format_arrays::matrix_to_string_ndarray(&l));
        println!("U =\n{}", format_arrays::matrix_to_string_ndarray(&u));

        start_time.elapsed()
    }
}

impl FactorizationLUMethod {
    pub fn final_resolution(&self) -> (Array2<f64>, Array2<f64>) {
        let a = Self::matrix(&self);
        let n = a.nrows();

        // Initialize L as identity matrix and U as zero matrix
        let mut l = Array2::<f64>::eye(n);
        let mut u = Array2::<f64>::zeros((n, n));
            
        for i in 0..n {
            println!();
            format_space::space("-", 100);
            println!("📋 Iteration i = {}", i);

            // Compute U (upper triangular matrix)
            for j in i..n {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l[[i, k]] * u[[k, j]];
                }
                u[[i, j]] = a[[i, j]] - sum;

                println!(
                    "U[{}, {}] = A[{}, {}] - Σ(L[{}, k] * U[k, {}]) = {:.4} - {:.4} = {:.4}",
                    i, j, i, j, i, j, a[[i, j]], sum, u[[i, j]]
                );
            }

            // Compute L (lower triangular matrix)
            for j in (i + 1)..n {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l[[j, k]] * u[[k, i]];
                }
                l[[j, i]] = (a[[j, i]] - sum) / u[[i, i]];

                println!(
                    "L[{}, {}] = (A[{}, {}] - Σ(L[{}, k] * U[k, {}])) / U[{}, {}] = ({:.4} - {:.4}) / {:.4} = {:.4}",
                    j, i, j, i, j, i, i, i, a[[j, i]], sum, u[[i, i]], l[[j, i]]
                );
            }

            // Print current state of L and U after this iteration
            println!("\n🔢 Matrix L so far:\n{}", format_arrays::matrix_to_string_ndarray(&l));
            println!("🔢 Matrix U so far:\n{}", format_arrays::matrix_to_string_ndarray(&u));
        }

        (l, u)
    }
}