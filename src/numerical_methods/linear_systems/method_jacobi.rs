use ndarray::{Array1};
use std::time::{Instant, Duration};
use crate::utils::{format_input, format_arrays, format_space};
use crate::{numerical_methods::linear_systems::strategy::LinearEquationSystemStrategy};
pub struct JacobiMethod;

/// Jacobi Method for solving linear systems Ax = b.
impl LinearEquationSystemStrategy for JacobiMethod {
    fn name(&self) -> &str { "JACOBI" }

    fn execute(&self) -> Duration {
        self.print_header();
        self.print_equation();
        
        println!();
        format_space::space("-", 100);
        println!("The tolerance for this system of equations is = {} ", Self::tolerance(&self));
        let max_iter = format_input::read_u32("Enter maximum number of iterations: ");
            
        let start_time = Instant::now();
        let solution = self.final_resolution(Self::tolerance(&self), max_iter as usize);
        
        format_space::space("-", 100);
        println!("\n🔢 Approximate solution found:");
        
        println!("x = {}", format_arrays::vector_to_string_ndarray(&&solution));
        start_time.elapsed()
    }
}

impl JacobiMethod {
    pub fn final_resolution(&self, tol: f64, max_iter: usize) -> Array1<f64> {
        let a = self.matrix();
        let b = self.vector();
        let n = a.nrows();

        let mut x = Array1::<f64>::zeros(n);
        let mut x_new = x.clone();

        for iter in 0..max_iter {
            for i in 0..n {
                let mut sum = 0.0;
                for j in 0..n {
                    if i != j {
                        sum += a[[i, j]] * x[j];
                    }
                }
                x_new[i] = (b[i] - sum) / a[[i, i]];
            }

            let error = (&x_new - &x).iter().map(|val| val.abs()).fold(0f64, f64::max);

            println!();
            format_space::space("-", 100);
            
            println!(
                "📋 Iter {:>2}: x = {}, error = {:.6}",
                iter + 1,
                format_arrays::vector_to_string_ndarray(&x_new),
                error
            );

            if error < tol {
                println!("\n✔️  Converged in {} iterations", iter + 1);
                return x_new;
            }

            x.assign(&x_new);
        }

        println!("\n⚠️ No converged after {} iterations", max_iter);
        x_new
    }
}