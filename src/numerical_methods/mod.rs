use std::time::Duration;

pub mod menu;
pub mod method_bisection;
pub mod method_newton_raphson;
pub mod method_secant;

trait NumericalMethodStrategy {
    
    fn execute(&self) -> Duration;

    fn name(&self) -> &str;

    fn print_function(&self) -> &str;
    
    fn tolerance(&self) -> f64 { 1e-6 }

    fn print_header(&self) {
        println!();
        println!("------------------------------------------");
        println!("⚙️  {} METHOD | {}", self.name(), self.print_function());
    }
}