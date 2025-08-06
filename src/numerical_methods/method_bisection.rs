use std::time::Instant;
use crate::utils::assistant;

/// f(x) = x * sin(x)
pub fn modulated_function(x: f64) -> f64 { x * x.sin() }


/// Bisection method implementation
pub fn bisection_method(f: fn(f64) -> f64, mut a: f64, mut b: f64, tol: f64, max_iter: u32) -> Option<f64> {
    
    if a > b { std::mem::swap(&mut a, &mut b); }

    if f(a) * f(b) >= 0.0 {
        println!("The bisection method cannot be applied.");
        println!("f(a) and f(b) must have opposite signs.");
        return None;
    }

    let mut iter = 0;
    while (b - a) / 2.0 > tol && iter < max_iter {
        let c = (a + b) / 2.0;
        let fc = f(c);

        println!("Iteration {}: c = {:.6}, f(c) = {:.6}", iter + 1, c, fc);

        if fc.abs() < tol { return Some(c); }

        if f(a) * fc < 0.0 { b = c; } else { a = c; }

        iter += 1;
    }

    Some((a + b) / 2.0)
}


pub fn execution() {
    let tol = 1e-6;

    println!();
    
    println!("------------------------------------------");  
    
    println!("⚙️  BISECTION METHOD | f(x) = x * sin(x)");
   
    let a = assistant::read_f64("Enter the value of a: ");

    let b = assistant::read_f64("Enter the value of b: ");
      
    let max_iter = assistant::read_u32("Enter the maximum number of iterations: ");

    
    let start_time = Instant::now();

    match bisection_method(modulated_function, a, b, tol, max_iter) {
        Some(root) => println!("\nThe approximate root is: {:.6}", root),
        None => println!("\nCould not find a root in the given interval."),
    }
    
    let elapsed = start_time.elapsed();

    println!("Execution time: {:.6} seconds", elapsed.as_secs_f64());

}