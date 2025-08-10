use crate::utils::format_input;

use crate::numerical_methods::root_finding::{
    method_bisection::BisectionMethod,
    method_newton_raphson::NewtonRaphsonMethod,
    method_secant::SecantMethod,
    strategy::NonlinearEquationsStrategy,
};

use crate::numerical_methods::linear_systems::{
    method_gauss::GaussMethod,
    method_factorization::FactorizationLUMethod,
    method_jacobi::JacobiMethod,
    strategy::LinearEquationSystemStrategy,
};

use crate::numerical_methods::interpolation_and_regression::{
    interpolation_lagrange::LagrangeInterpolation,
    interpolation_newton::NewtonInterpolation,
    strategy::InterpolationStrategy,
};


pub fn implementation() {
    println!("\n{}", "=".repeat(100));  
    println!("📂 SECONDARY MENU - NUMERICAL METHODS");
    println!("1. Bisection Method");
    println!("2. Newton-Raphson Method");
    println!("3. Secant Method");
    println!("4. Gaussian Elimination");
    println!("5. LU Decomposition");
    println!("6. Jacobi Method");
    println!("7. Lagrange Interpolation");
    println!("8. Newton Interpolation");
    println!("0. Exit");

    let choice = format_input::read_u32("Enter your choice (0-6): ");

    // Define a trait object that can be either type
    enum StrategyType {
        Nonlinear(Box<dyn NonlinearEquationsStrategy>),
        Linear(Box<dyn LinearEquationSystemStrategy>),
        Interpolation(Box<dyn InterpolationStrategy>)
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

        // Interpolation and regression
        7 => Some(StrategyType::Interpolation(Box::new(LagrangeInterpolation))),
        8 => Some(StrategyType::Interpolation(Box::new(NewtonInterpolation))),

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
            StrategyType::Interpolation(method) => method.execute(),
        };
        println!("Execution time: {:.6} seconds", time.as_secs_f64());
        println!("{}", "-".repeat(100));
    }
}