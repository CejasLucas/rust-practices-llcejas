mod utils;
mod numerical_methods;
mod sorting_algorithms;

/// RUST 🦀
fn main() {
    println!();
    println!("==========================================");  
    println!("🗂️  MAIN SYSTEM MENU IN RUST");
    println!("⏭︎  Select an option.");
    println!("1. Numerical Methods");
    println!("2. Sorting Algorithms");
    println!("3. Concurrence");

    let choice = utils::assistant::read_u32("Enter your choice (1-3): ");

    match choice {
        1 => numerical_methods::menu::menu(), 
        2 => sorting_algorithms::menu::menu(),
        3 => println!("Concurrence menu not implemented yet."),
        _ => println!("Invalid option. Please enter 1 to 3."),
    }
}