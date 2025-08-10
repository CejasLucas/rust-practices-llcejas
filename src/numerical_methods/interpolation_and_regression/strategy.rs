use std::time::Duration;

use ndarray::{array, Array1};

pub trait InterpolationStrategy {
    fn name(&self) -> &str;

    fn execute(&self) -> Duration;
    
    fn vector_of_x(&self) -> Array1<f64> {
        array![50.0, -24.5, 17.0, -53.0, 75.0, 93.0]
    } 

    fn vector_of_y(&self) -> Array1<f64> {
        array![-85.0, 57.0, -0.5, 120.0, 13.0, 59.0]
    } 

    fn print_header(&self) {
        println!("\n{}", "-".repeat(100));
        println!("⚙️ INTERPOLATION - {} POLINOMIAL", self.name());
        println!("Axis points (x) = {:?} \n", self.vector_of_x());
        println!("Axis points (y) = {:?} \n", self.vector_of_y());
    }
}