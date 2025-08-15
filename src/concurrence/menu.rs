use crate::{concurrence, utils::{ format_input, format_space}};

pub fn implementation() {
    println!();
    format_space::space("=", 100);        
    println!("🗂️  SECONDARY MENU - CONCURRENCE");
    println!(" 1. Producer and Consumer");
    println!(" 2. Shared Counter");
    println!(" 3. Elevator Simulation");
    println!(" 4. Task Scheduler");
    println!(" 5. Semaphore Crossing");
    println!(" 0. Exit");

    let choice = format_input::read_u32("Enter your choice (1-5): ");
    println!();
    
    match choice {
        1 => concurrence::example_producer_consumer::run(), 
        2 => concurrence::example_shared_counter::run(),
        3 => concurrence::example_elevator_simulation::run(),
        4 => concurrence::example_task_scheduler::run(),
        5 => concurrence::example_semaphore_crossing::run(),
        _ => println!("Invalid option. Please enter 1 to 5."),
    }
}