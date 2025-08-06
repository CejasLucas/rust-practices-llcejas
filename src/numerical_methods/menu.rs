use crate::utils::assistant;
use crate::numerical_methods::method_bisection;
use crate::numerical_methods::method_newton_raphson;
use crate::numerical_methods::method_secant;

pub fn menu() {
    println!();
    println!("==========================================");  
    println!("📂 SECONDARY MENU - NUMERICAL METHOD");
    println!("↪︎ Select an option.");
    println!("1. Bisection Method");
    println!("2. Newton-Raphson Method");
    println!("3. Secant Method");

    let choice = assistant::read_u32("Enter your choice (1-3): ");
    
    match choice {
        1 => method_bisection::execution(),
        2 => method_newton_raphson::execution(),
        3 => method_secant::execution(),
        _ => println!("Invalid choice. Please enter a number from 1 to 3."),
    }

}