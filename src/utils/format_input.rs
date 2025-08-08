use std::io::{self, Write};

/// Helper function to read a floating-point number from stdin
pub fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    loop {
        print!("{}", prompt);                      
        io::stdout().flush().unwrap();             
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Invalid input. Please enter a valid floating-point number."),
        }
    }
}

/// Helper function to read an unsigned integer from stdin
pub fn read_u32(prompt: &str) -> u32 {
    let mut input = String::new();
    loop {
        print!("{}", prompt);                      
        io::stdout().flush().unwrap();             
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Invalid input. Please enter a valid positive integer."),
        }
    }
}