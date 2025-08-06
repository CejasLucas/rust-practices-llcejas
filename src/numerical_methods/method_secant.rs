use std::time::Instant;
use crate::utils::assistant;

/// f(x) = x³ - x - 1
pub fn modulated_function(x: f64) -> f64 { x.powi(3) - x - 1.0 }

/// Secant method implementation
pub fn method_secant(mut x0: f64, mut x1: f64, tol: f64, max_iter: u32) -> Option<f64> {
    for iter in 0..max_iter {
        
        let fx0 = modulated_function(x0);
        
        let fx1 = modulated_function(x1);

        if (fx1 - fx0).abs() < 1e-12 { println!("Denominator too close to zero at iteration {}. Method may not converge.", iter + 1); return None; }

        let x2 = x1 - fx1 * (x1 - x0) / (fx1 - fx0);

        println!("Iteration {:>2}: x = {:>12.6}, f(x) = {:>12.6}", iter + 1, x2, modulated_function(x2));

        if (x2 - x1).abs() < tol { return Some(x2); }

        x0 = x1;

        x1 = x2;
    }
    println!("Maximum number of iterations reached without convergence.");
    None
}

/// Execute secant method with user input
pub fn execution() {
    let tol = 1e-6;

    println!();

    println!("---------------------------------------");  
    
    println!("⚙️  SECANT METHOD | f(x) = x³ - x - 1");

    let x0 = assistant::read_f64("Enter first initial guess (x0): ");
    
    let mut x1 = assistant::read_f64("Enter second initial guess (x1): ");

    // Validate that x0 != x1 to avoid division by zero initially
    while (x1 - x0).abs() < 1e-12 {
        println!("Second initial guess must be different from the first. Please enter again.");
        x1 = assistant::read_f64("Enter second initial guess (x1): ");
    }

    let max_iter = assistant::read_u32("Enter maximum number of iterations: ");

    let start_time = Instant::now();

    match method_secant(x0, x1, tol, max_iter) {
        Some(root) => println!("\nApproximate root found: {:.6}", root),
        None => println!("\nSecant method did not converge."),
    }
    
    let elapsed = start_time.elapsed();

    println!("Execution time: {:.6} seconds", elapsed.as_secs_f64());

}