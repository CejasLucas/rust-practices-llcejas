mod utils;
mod concurrence;
mod numerical_methods;
mod sorting_algorithms;

/// RUST 🦀
fn main() {
    utils::format_space::space("=", 100);
    println!("💼 LOGIN MENU - SYSTEM MADE RUST");
    println!("⏭︎  Select an option.");
    println!("1. Numerical Methods");
    println!("2. Sorting Algorithms");
    println!("3. Concurrence");

    let choice = utils::format_input::read_u32("Enter your choice (1-3): ");

    match choice {
        1 => numerical_methods::menu::implementation(), 
        2 => sorting_algorithms::menu::implementation(),
        3 => concurrence::menu::implementation(),
        _ => println!("Invalid option. Please enter 1 to 3."),
    }
}