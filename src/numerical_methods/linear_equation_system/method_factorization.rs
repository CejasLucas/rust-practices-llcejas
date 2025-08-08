use ndarray::{Array2};
use std::time::{Instant, Duration};
use crate::{numerical_methods::linear_equation_system::LinearEquationSystemStrategy, utils::format_arrays};

pub struct FactorizationLUMethod;

impl LinearEquationSystemStrategy for FactorizationLUMethod {
    fn name(&self) -> &str {
        "FACTORIZATION LU"
    }

    fn execute(&self) -> Duration {
        self.print_header();
        self.print_equation();

        let start_time = Instant::now();
        let (l, u) = self.final_resolution(); 
        println!("\n✅ Solución");

        println!("L = {}", format_arrays::matrix_to_string_ndarray(&l));
        println!("U = {}", format_arrays::matrix_to_string_ndarray(&u));
        start_time.elapsed()
    }
}

impl FactorizationLUMethod {
    pub fn final_resolution(&self) -> (Array2<f64>, Array2<f64>) {
        let a = Self::matrix(&self);
        let n = a.nrows();

        let mut l = Array2::<f64>::eye(n);
        let mut u = Array2::<f64>::zeros((n, n));

        for i in 0..n {
            for j in i..n {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l[[i, k]] * u[[k, j]];
                }
                u[[i, j]] = a[[i, j]] - sum;
            }

            for j in (i + 1)..n {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l[[j, k]] * u[[k, i]];
                }
                l[[j, i]] = (a[[j, i]] - sum) / u[[i, i]];
            }
        }

        (l, u)
    }
}