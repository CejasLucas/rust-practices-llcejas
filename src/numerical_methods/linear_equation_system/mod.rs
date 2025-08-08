pub mod method_factorization;
pub mod method_gauss;
pub mod method_jacobi;

use std::time::Duration;
use crate::utils::format_arrays;
use ndarray::{array, Array1, Array2};

pub trait LinearEquationSystemStrategy {
    fn execute(&self) -> Duration;

    fn name(&self) -> &str;

    fn tolerance(&self) -> f64 { 1e-6 }
    
    fn print_template(&self) -> &str { "Ax = b" }

    fn vector(&self) -> Array1<f64> {
        array![15.0, 10.0, 10.0]
    }

    fn matrix(&self) -> Array2<f64> {
        array![
            [4.0, -1.0, 0.0],
            [-1.0, 4.0, -1.0],
            [0.0, -1.0, 3.0],
        ]
    }

    fn print_equation(&self) {
        let matrix_str = format_arrays::matrix_to_string_ndarray(&self.matrix());
        let vector_str = format_arrays::vector_to_string_ndarray(&&self.vector());
        println!("A = {} ", matrix_str);
        println!("b = {} ", vector_str)
    }

    fn print_header(&self) {
        println!();
        println!("-------------------------------------------------------------\n");
        println!("⚙️  {} METHOD | {}", self.name(), self.print_template());
    }
}