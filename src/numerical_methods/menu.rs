use crate::utils::format_input;

use crate::numerical_methods::nonlinear_equations::{
    method_bisection::BisectionMethod,
    method_newton_raphson::NewtonRaphsonMethod,
    method_secant::SecantMethod,
    NonlinearEquationsStrategy,
};

use crate::numerical_methods::linear_equation_system::{
    method_gauss::GaussMethod,
    method_factorization::FactorizationLUMethod,
    method_jacobi::JacobiMethod,
    LinearEquationSystemStrategy,
};

pub fn implementation() {
    println!("\n==========================================");  
    println!("📂 SECONDARY MENU - NUMERICAL METHODS");
    println!("1. Bisection Method");
    println!("2. Newton-Raphson Method");
    println!("3. Secant Method");
    println!("4. Gaussian Elimination");
    println!("5. LU Decomposition");
    println!("6. Jacobi Method");
    println!("0. Exit");

    let choice = format_input::read_u32("Enter your choice (0-6): ");

    // Define a trait object that can be either type
    enum StrategyType {
        Nonlinear(Box<dyn NonlinearEquationsStrategy>),
        Linear(Box<dyn LinearEquationSystemStrategy>),
    }

    let strategy: Option<StrategyType> = match choice {
        // Nonlinear methods
        1 => Some(StrategyType::Nonlinear(Box::new(BisectionMethod))),
        2 => Some(StrategyType::Nonlinear(Box::new(NewtonRaphsonMethod))),
        3 => Some(StrategyType::Nonlinear(Box::new(SecantMethod))),

        // Linear systems methods
        4 => Some(StrategyType::Linear(Box::new(GaussMethod))),
        5 => Some(StrategyType::Linear(Box::new(FactorizationLUMethod))),
        6 => Some(StrategyType::Linear(Box::new(JacobiMethod))),

        // Exit 
        0 => {
            println!("Exiting program.");
            return;
        }
        _ => {
            println!("Invalid option. Please enter a number from 0 to 6.");
            return;
        }
    };

    // Now execute based on which type it is
    if let Some(s) = strategy {
        let time = match s {
            StrategyType::Nonlinear(method) => method.execute(),
            StrategyType::Linear(method) => method.execute(),
        };
        println!("Execution time: {:.6} seconds", time.as_secs_f64());
        println!("-------------------------------------------------------------\n");
    }
}