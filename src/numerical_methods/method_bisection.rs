use std::time::{Instant, Duration};
use crate::utils::assistant;
use crate::numerical_methods::NumericalMethodStrategy;

pub struct BisectionMethod;

impl NumericalMethodStrategy for BisectionMethod {
    fn name(&self) -> &str { "BISECTION" }
    fn print_function(&self) -> &str { "f(x) = x * sin(x)" }

    fn execute(&self) -> Duration {
        self.print_header();

        let a = assistant::read_f64("Enter the value of a: ");
        let b = assistant::read_f64("Enter the value of b: ");
        let max_iter = assistant::read_u32("Enter the maximum number of iterations: ");

        let start_time = Instant::now();
        Self::find_root_interval(a, b, max_iter);
        start_time.elapsed()
    }
}

impl BisectionMethod {
    fn function(x: f64) -> f64 { x * x.sin() }
    fn tolerance() -> f64 { 1e-6 }

    fn find_root_interval(mut a: f64, mut b: f64, max_iter: u32) -> Option<f64> {
        if a > b { std::mem::swap(&mut a, &mut b); }

        if Self::function(a) * Self::function(b) >= 0.0 {
            println!("The bisection method cannot be applied.");
            println!("f(a) and f(b) must have opposite signs.");
            return None;
        }

        let mut iter = 0;
        while (b - a) / 2.0 > Self::tolerance() && iter < max_iter {
            let c = (a + b) / 2.0;
            let fc = Self::function(c);
            println!("Iteration {}: c = {:.6}, f(c) = {:.6}", iter + 1, c, fc);

            if fc.abs() < Self::tolerance() {
                return Some(c);
            }

            if Self::function(a) * fc < 0.0 {
                b = c;
            } else {
                a = c;
            }

            iter += 1;
        }
        Some((a + b) / 2.0)
    }
}