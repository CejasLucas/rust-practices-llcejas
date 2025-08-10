use std::time::Duration;
use crate::utils::format_space;
pub trait NonlinearEquationsStrategy {    
    
    fn name(&self) -> &str;

    fn execute(&self) -> Duration;

    fn print_function(&self) -> &str;
    
    fn tolerance(&self) -> f64 { 1e-6 }

    fn print_header(&self) {
        println!();
        format_space::space("=", 100);
        println!("⚙️  {} METHOD | {}", self.name(), self.print_function());
    }
}