use std::time::Instant;
use crate::utils::assistant;

/// f(x) = x³ - x - 2
pub fn modulated_function(x: f64) -> f64 { x.powi(3) - x - 2.0 }

/// f′(x) = 3x² - 1
pub fn modulated_function_derivative(x: f64) -> f64 { 3.0 * x.powi(2) - 1.0 }

/// Perform one iteration of Newton-Raphson
pub fn newton_raphson_step(x: f64) -> Option<f64> {
    
    let fx = modulated_function(x);
    
    let fpx = modulated_function_derivative(x);

    if fpx.abs() < 1e-12 { println!("Derivative too close to zero at x = {:.6}. Method will not converge.", x); return None; }

    Some(x - fx / fpx)
}


/// Newton-Raphson method implementation
pub fn method_newton_raphson(mut x0: f64, tol: f64, max_iter: u32) -> Option<f64> {
    for iter in 0..max_iter {
        if let Some(x1) = newton_raphson_step(x0) {
    
            println!("Iteration {:>2}: x = {:>12.6}, f(x) = {:>12.6}", iter + 1, x1, modulated_function(x1));

            if (x1 - x0).abs() < tol { return Some(x1); }
            
            x0 = x1;

        } else {
            return None;
        }
    }
    println!("Maximum iterations reached without convergence.");
    None
}


/// Execute method with user input
pub fn execution() {
    let tol = 1e-6;

    println!();
    
    println!("-----------------------------------------------");  
    
    println!("⚙️  NEWTON RAPHSON METHOD | f(x) = x³ - x - 2");
   
    let x0 = assistant::read_f64("Enter initial guess (x0): ");

    let max_iter = assistant::read_u32("Enter maximum number of iterations: ");

    let start_time = Instant::now();

    match method_newton_raphson(x0, tol, max_iter) {
        Some(root) => println!("\nApproximate root found: {:.6}", root),
        None => println!("\nNewton-Raphson method did not converge."),
    }

    let elapsed = start_time.elapsed();

    println!("Execution time: {:.6} seconds", elapsed.as_secs_f64());

}