use crate::utils::assistant;
use crate::numerical_methods::{
    method_bisection::BisectionMethod,
    method_newton_raphson::NewtonRaphsonMethod,
    method_secant::SecantMethod,
    NumericalMethodStrategy, 
};

pub fn implementation() {
    println!();
    println!("==========================================");  
    println!("📂 SECONDARY MENU - NUMERICAL METHOD");
    println!("↪︎ Select an option.");
    println!("1. Bisection Method");
    println!("2. Newton-Raphson Method");
    println!("3. Secant Method");

    let choice = assistant::read_u32("Enter your choice (1-3): ");

    let strategy: Box<dyn NumericalMethodStrategy> = match choice {
        1 => Box::new(BisectionMethod),
        2 => Box::new(NewtonRaphsonMethod),
        3 => Box::new(SecantMethod),
        _ => {
            println!("Invalid choice. Please enter 1 to 3.");
            return;
        }
    };

    let time = strategy.execute();

    println!("Execution time: {:.6} seconds", time.as_secs_f64());
    println!("---------------------------------------------------------------------------------\n");
}