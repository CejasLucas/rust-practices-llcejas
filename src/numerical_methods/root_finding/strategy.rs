use std::time::Duration;

pub trait NonlinearEquationsStrategy {    
    
    fn name(&self) -> &str;

    fn execute(&self) -> Duration;

    fn print_function(&self) -> &str;
    
    fn tolerance(&self) -> f64 { 1e-6 }

    fn print_header(&self) {
        println!("\n{}", "-".repeat(100));
        println!("⚙️  {} METHOD | {}", self.name(), self.print_function());
    }
}